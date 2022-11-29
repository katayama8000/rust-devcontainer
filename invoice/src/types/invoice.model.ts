export type invoiceType = {
  billerCity: string;
  billerCountry: string;
  billerStreetAddress: string;
  billerZipCode: string;
  clientCity: string;
  clientCountry: string;
  clientEmail: string;
  clientName: string;
  clientStreetAddress: string;
  clientZipCode: string;
  docId: string;
  invoiceDate: string;
  invoiceDateUnix: number;
  invoiceDraft: boolean;
  invoiceId: string;
  invoiceItemList: {
    price: string;
    qty: string;
    itemName: string;
    id: string;
    total: number;
  }[];

  invoicePaid: boolean;
  invoicePending: boolean;
  invoiceTotal: number;
  paymentDueDate: string;
  paymentDueDateUnix: number;
  paymentTerms: string;
  productDescription: string;
};
