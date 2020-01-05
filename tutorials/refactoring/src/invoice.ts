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

interface Invoice {
  customer: string;
  performances: Performance[];
}

export function statement(invoice: Invoice, plays: Plays): void {
  let totalAmount: number = 0;
  let volumeCredits: number = 0;
  let result = `Statement for ${invoice.customer}`;
  console.log(result);
}
