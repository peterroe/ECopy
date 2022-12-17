import chokidar from 'chokidar'
import { resolve } from 'node:path'
import c from 'node:child_process'
// One-liner for current directory
chokidar.watch('./src/*.rs', {
  ignored: 'node_modules'
}).on('change', (path) => {
  const target = resolve(process.cwd(), path)
  console.log(target);
  c.exec(`rustfmt ${target}`)
});