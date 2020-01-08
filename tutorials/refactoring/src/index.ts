import { statement, Invoice } from "./invoice";
const plays = require("./playes.json");
const invoices = require("./invoices.json");


console.log(JSON.stringify(plays, null, "  "));
console.log(JSON.stringify(invoices, null, "  "));
invoices.forEach((invoice: Invoice) => {
  console.log("Invoice:, ", invoice);

  const ret = statement(invoice, plays);
  console.log(ret);
});
