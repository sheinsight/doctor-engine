Object.keys(envars)
  .map(envar => `${envar}=${envars[envar]}`)
  .join(' ')
  |> `$ ${^^}`
  |> chalk.dim(^^, 'node', args.join(' '))
  |> console.log(^^);