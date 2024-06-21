const path = require('path');
const programDir = path.join(__dirname, '..');
const idlDir = path.join(__dirname, 'idl');
const sdkDir = path.join(__dirname, 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'shank',
  programName: 'stockpile_lite',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};