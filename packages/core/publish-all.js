const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const rootDir = path.join(__dirname, 'npm');
const args = process.argv.slice(2);

fs.readdirSync(rootDir, { withFileTypes: true })
  .filter((dirent) => dirent.isDirectory())
  .forEach((dirent) => {
    const isPre = args.includes('--pre');
    const dirPath = `${rootDir}/${dirent.name}`;
    console.log(`Publishing ${dirPath}...`);
    try {
      execSync(`npm publish ${isPre ? '--tag beta' : ''} --access public`, {
        cwd: dirPath,
        stdio: 'inherit',
      });
    } catch (error) {
      console.error(`Error publishing ${dirPath}: ${error.message}`);
    }
  });
