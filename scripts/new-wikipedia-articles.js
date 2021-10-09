// This is a proof of concept for now, this eventually should probably be
// rewritten to Rust to keep the repo all Rust?

import fetch from 'node-fetch';

const { WIKI_END_DATE, WIKI_LOCALE } = process.env;
const END_DATE = new Date(WIKI_END_DATE).toISOString();

if (!WIKI_END_DATE || !WIKI_LOCALE) {
  console.error('WIKI_END_DATE and WIKI_LOCALE need to be present in ENV');
  process.exit(1);
  return;
}

let numberOfArticles = 0;

async function queryNewArticles(beginDate = new Date().toISOString(), lecontinue) {
  let url = `https://${WIKI_LOCALE}.wikipedia.org/w/api.php?action=query&format=json&list=logevents&leprop=title%7Ctimestamp&letype=create&lestart=${beginDate}&leend=${END_DATE}&lenamespace=0&lelimit=500`;
  if (lecontinue) {
    url += `&lecontinue=${lecontinue}`;
  }
  console.error('Querying:', url);

  const response = await fetch(url);
  const data = await response.json();

  if (!data || !data.query || !data.query.logevents || data.query.logevents.length === 0) {
    console.error('NO_DATA', data);
    return;
  }

  const logEvents = data.query.logevents;

  console.error('>>>> Currently at:', logEvents[0].timestamp);

  logEvents.forEach(logevent => {
    console.log(logevent.title);
  });

  numberOfArticles += logEvents.length;

  const forceNewRequest = numberOfArticles % 100_000 === 0;

  if (data.continue && data.continue.lecontinue && !forceNewRequest) {
    console.error('NEW_ARTICLES', numberOfArticles);
    queryNewArticles(beginDate, data.continue.lecontinue);
    return;
  } else {
    // We need to issue a new Query to see if we have more
    console.error('Forcing new request...');
    const newBeginDate = new Date(logEvents[logEvents.length - 1].timestamp).toISOString();
    queryNewArticles(newBeginDate);
  }
}

console.error('Starting to fetch new article titles created since ..', END_DATE)
queryNewArticles();
