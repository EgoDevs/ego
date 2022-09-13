import mockUsers from './identities.json';

// userGroup
// 0:admin
// 1:developer
// 2:operator
// 3:endUser

export const admins = mockUsers.filter(i => {
  if (i.userGroup === 0) return i;
});
export const developers = mockUsers.filter(i => {
  if (i.userGroup === 1) return i;
});
export const operators = mockUsers.filter(i => {
  if (i.userGroup === 2) return i;
});
export const endUsers = mockUsers.filter(i => {
  if (i.userGroup === 3) return i;
});
