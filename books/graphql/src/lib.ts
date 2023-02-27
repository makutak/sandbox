const requestGithubToken = async (credentials: string) => {
  const params = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Accept: 'application/json',
    },
    body: JSON.stringify(credentials),
  };

  try {
    const res = await fetch(`https://github.com/login/oauth/access_token`, params);
    return await res.json();
  } catch (err) {
    throw new Error(JSON.stringify(err));
  }
}

const requestGithubUserAccount = async (token: string) => {
  try {
    const res = await fetch(`https://api.github.com/user?access_token${token}`);
    return await res.json();
  } catch (err) {
    throw new Error(JSON.stringify(err));
  }
}

const authrizeWithGithub = async (credentials: string) => {
  const { access_token } = await requestGithubToken(credentials);
  const githubUser = await requestGithubUserAccount(access_token);
  return { ...githubUser, access_token };
}
