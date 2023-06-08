const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const rootDir = path.join(__dirname, 'npm');

fs.readdirSync(rootDir, { withFileTypes: true })
  .filter((dirent) => dirent.isDirectory())
  .forEach((dirent) => {
    const dirPath = `${rootDir}/${dirent.name}`;
    console.log(`Publishing ${dirPath}...`);
    try {
      execSync(
        'npm publish --access public --registry https://registry.npmjs.org/',
        { cwd: dirPath, stdio: 'inherit' }
      );
    } catch (error) {
      console.error(`Error publishing ${dirPath}: ${error.message}`);
    }
  });
