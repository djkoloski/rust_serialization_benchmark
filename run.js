#!/usr/bin/env node

import fs from 'fs/promises';
import format from './format.js';
import {
  dirname,
  join
} from 'path'

const dir = dirname(new URL(
  import.meta.url).pathname);

const result = join(dir, 'result.');

const txt = await fs.readFile(result + 'log', 'utf8');

await fs.writeFile(
  result + 'md',
  format(txt)
)
