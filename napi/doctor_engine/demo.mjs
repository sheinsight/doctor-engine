

import { innerDebugLint,innerLint, NaPiCategory } from './index.js'

(async () => {
  console.log('start')
const res = await innerLint({
  cwd: '/Users/10015448/Git/drawio_ui',
  verbose: false,
}, NaPiCategory.V20250601Inner).catch(e => {
  console.log(e)
})

  console.log('end')
console.log(res);
})()