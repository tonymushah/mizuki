const pkgName = process.argv[2]
const tag = process.argv[3]

async function packageExists(name, tag) {
  let request = await fetch(`https://registry.npmjs.com/${name}`)
  if (request.status === 404) {
    return 'not published'
  } else if (request.status < 400) {
    let response = await request.json()
    return response['dist-tags'][tag]
  } else {
    throw new Error('request error')
  }
}

async function main() {
  const pkg = await packageExists(pkgName, tag)
  console.log(pkg)
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
