const plays = require("./playes.json");
const invoices = require("./invoices.json");

interface Play {
  name: string;
  type: string;
}

interface Plays {
  subject: Play;
}

interface Performance {
  playID: string;
  audience: number;
}

export interface Invoice {
  customer: string;
  performances: Performance[];
}

function playFor(aPerformance: Performance) {
  return plays[aPerformance.playID];
}

export function statement(invoice: Invoice, plays: Plays): string {
  let totalAmount: number = 0;
  let volumeCredits: number = 0;
  let result = `Statement for ${invoice.customer}\n`;


  const format = new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2
  }).format;

  for (let perf of invoice.performances) {
    let thisAmount: number = amountFor(perf, playFor(perf));

    // ボリューム特典のポイントを加算
    volumeCredits += Math.max(perf.audience - 30, 0);

    // 喜劇のときは10人につき、さらにポイントを追加
    if ("comedy" === playFor(perf).type) {
      volumeCredits += Math.floor(perf.audience / 5);
    }

    // 注文の内訳を出力
    result += ` ${playFor(perf).name}: ${format(thisAmount / 100)} (${perf.audience} seats) \n`;
    totalAmount += thisAmount;
  }
  result += `Amount owed is ${format(totalAmount / 100)}\n`;
  result += `You earned ${volumeCredits} credits\n`;
  return result;
}

export function amountFor(aPerformance: Performance, play: Play): number {
  let result: number = 0;

  switch (playFor(aPerformance).type) {
    case "tragedy":
      result = 40000;
      if (aPerformance.audience > 30) {
        result += 1000 * (aPerformance.audience - 30);
      }
      break;
    case "comedy":
      result = 30000;
      if (aPerformance.audience > 20) {
        result += 10000 + 500 * (aPerformance.audience - 20);
      }
      result += 300 * aPerformance.audience;
      break;
    default:
      throw new Error(`unknown type: ${playFor(aPerformance).type}`);
  }

  return result;
}

export function exec() {
  console.log(JSON.stringify(plays, null, "  "));
  console.log(JSON.stringify(invoices, null, "  "));
  invoices.forEach((invoice: Invoice) => {
    console.log("Invoice:, ", invoice);

    const ret = statement(invoice, plays);
    console.log(ret);
  });
}
