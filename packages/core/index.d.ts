/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface GitInfo {
  username: string
  email: string
  sshUrl: string
  userRepo: string
  currentBranch: string
}
export function getGitInfo(): GitInfo