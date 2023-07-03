import child_process from 'child_process';
import fs from 'fs';

function formatTemplate(str, args) {
  for (const arg in args) {
    str = str.replace(`\${${arg}}`, args[arg])
  }
  return str
}

function main() {
  const now = new Date()
  const date = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`

  const metadata = JSON.parse(child_process.execSync(`cargo metadata`, {
    maxBuffer: 4 * 1024 * 1024,
  }).toString('utf-8'))

  const results = child_process.execSync(`cargo bench`).toString('utf-8')
  console.log(results)
  const resultsPath = `benchmark_results/${date}.txt`
  fs.writeFileSync(resultsPath, results)

  const template = fs.readFileSync('README.md.template', 'utf-8')
  const DO_NOT_EDIT = '<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->\n<!-- edit README.md.template instead -->\n\n'
  fs.writeFileSync(
    'README.md',
    DO_NOT_EDIT + formatTemplate(template, { date, results: format(results, metadata) })
  )
}

function parseUnit(unit) {
  switch (unit) {
    case 'ns':
      return 1;
    case 'µs':
      return 1000;
    case 'ms':
      return 1000000;
  }
}

function parseTime(time) {
  return Number(time.substr(0, time.length - 3)) * parseUnit(time.substr(time.length - 2, 2))
}

function getMinValue(dataset, bench) {
  let min = null
  for (let crate in dataset) {
    if (dataset[crate][bench] !== undefined) {
      let benchResults = dataset[crate][bench]
      if (benchResults.value != null && (min == null || benchResults.value < min)) {
        min = benchResults.value
      }

      for (let variant in benchResults.variants) {
        let variantResults = benchResults.variants[variant]
        if (variantResults.value != null && (min == null || variantResults.value < min)) {
          min = variantResults.value
        }
      }
    }
  }
  return min
}

function buildTables(results, dataset, crates, columns, footnote) {
  let header = '| Crate |'
  for (let column of columns) {
    header += ` ${column[0].toUpperCase() + column.substr(1)} |`
  }
  header += '\n|---|'
  for (let column of columns) {
    header += '--:|'
  }

  let dataTable = ''
  for (let crate in results[dataset]) {
    let hadResult = false
    let row = `| [${crate} ${crates[crate]}][${crate}] |`
    for (let column of columns) {
      let output = results[dataset][crate][column]
      if (output === undefined) {
        row += ` ${footnote} |`
      } else {
        if (output.display != null) {
          row += ` ${output.display}`
          hadResult = true
        }
        for (let variant in output.variants) {
          row += ` <span title="${variant}">*${output.variants[variant].display}\\**</span>`
          hadResult = true
        }
        row += ' |'
      }
    }
    if (hadResult) {
      if (dataTable != '') {
        dataTable += '\n'
      }
      dataTable += `${row}`
    }
  }

  let comparisonTable = ''
  for (let crate in results[dataset]) {
    let hadResult = false
    let row = `| [${crate} ${crates[crate]}][${crate}] |`
    for (let column of columns) {
      let min = getMinValue(results[dataset], column)
      let output = results[dataset][crate][column]
      if (output === undefined) {
        row += ` ${footnote} |`
      } else {
        if (output.value != null) {
          row += ` ${(min / output.value * 100).toFixed(2)}%`
          hadResult = true
        }
        for (let variant in output.variants) {
          row += ` <span title="${variant}">*${(min / output.variants[variant].value * 100).toFixed(2)}%\\**</span>`
          hadResult = true
        }
        row += ' |'
      }
    }
    if (hadResult) {
      if (comparisonTable != '') {
        comparisonTable += '\n'
      }
      comparisonTable += `${row}`
    }
  }

  return {
    header,
    data: dataTable,
    comparison: comparisonTable
  }
}

const DATASET_DESCRIPTIONS = {
  log: 'This data set is composed of HTTP request logs that are small and contain many strings.',
  mesh: 'This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.',
  mk48: 'This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.',
  minecraft_savedata: 'This data set is composed of Minecraft player saves that contain highly structured data.'
}

function format(input, metadata) {
  let bench_times_re = /^([a-z0-9_\-]+)\/([a-z_\-]+)\/([a-z\-]+)(?: \(([a-z \-+]*)\))?\W+time:   \[\d+\.\d+ [µnm]s (\d+\.\d+ [µnm]s).*$/gm
  let bench_sizes_re = /^([a-z0-9_\-]+)\/([a-z_\-]+)\/(size|zlib|zstd) (\d+)$/gm

  let results = {}
  let crates = {}
  for (let match of input.matchAll(bench_times_re)) {
    let dataset = match[1]
    let crate = match[2]
    let bench = match[3]
    let variant = match[4]
    let time = match[5]

    if (results[dataset] === undefined) {
      results[dataset] = {}
    }

    if (results[dataset][crate] === undefined) {
      results[dataset][crate] = {}
    }

    if (results[dataset][crate][bench] === undefined) {
      results[dataset][crate][bench] = {
        display: null,
        value: null,
        variants: {}
      }
    }

    let benchResults = results[dataset][crate][bench]

    if (variant == null) {
      benchResults.display = time
      benchResults.value = parseTime(time)
    } else {
      benchResults.variants[variant] = {
        display: time,
        value: parseTime(time)
      }
    }

    if (!(crate in crates)) {
      crates[crate] = get_crate_version(crate, metadata)
    }
  }

  for (let match of input.matchAll(bench_sizes_re)) {
    let dataset = match[1]
    let crate = match[2]
    let bench = match[3]
    let size = match[4]

    results[dataset][crate][bench] = {
      display: Number(size),
      value: Number(size)
    }
  }

  let output = ''
  for (let dataset in results) {
    let serdeTables = buildTables(results, dataset, crates, ['serialize', 'deserialize', 'size', 'zlib', 'zstd'], '†')
    let zcdTables = buildTables(results, dataset, crates, ['access', 'read', 'update'], '‡')

    output += `\
## \`${dataset}\`

${DATASET_DESCRIPTIONS[dataset] || 'Missing dataset description'}

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

${serdeTables.header}
${serdeTables.data}

#### Zero-copy deserialization speed

${zcdTables.header}
${zcdTables.data}

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

${serdeTables.header}
${serdeTables.comparison}

#### Zero-copy deserialization speed

${zcdTables.header}
${zcdTables.comparison}

`
  }

  for (let crate in crates) {
    output += `[${crate}]: https://crates.io/crates/${crate}/${crates[crate]}\n`
  }
  output += `\n\n`

  return output
}

function get_crate_version(crate, metadata) {
  for (const pkg in metadata.packages) {
    let pkg_data = metadata.packages[pkg]
    if (pkg_data.name == crate) {
      return pkg_data.version
    }
  }
  throw new Error(`Failed to find a crate version for ${crate}`)
}

main()
