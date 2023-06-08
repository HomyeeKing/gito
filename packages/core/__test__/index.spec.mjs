import test from 'ava';

import { getGitInfo } from '../index.js';

test('get git info', (t) => {
  const info = getGitInfo();
  t.deepEqual(info, {
    username: 'HomyeeKing',
    email: 'HomyeeKing@gmail.com',
    sshUrl: 'git@github.com:HomyeeKing/gito.git',
    userRepo: 'HomyeeKing/gito',
    currentBranch: 'master',
  });
});
