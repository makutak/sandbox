const axios = require('axios');
const { URLSearchParams } = require('url');
const querystring = require('querystring');

const token = 'ここにtokenが入る';

const params = querystring.stringify({
    message: '送りたいメッセージ',
});

axios.post('https://notify-api.line.me/api/notify', params, {
    headers: {
        'Content-Type':'application/x-www-form-urlencoded',
        'Authorization': `Bearer ${token}`,
    },
}).then((res) => {
    console.log('succes!!');
    console.log(res.data);
}).catch((err) => {
    console.log('error!');
    console.log(err.response);
});
