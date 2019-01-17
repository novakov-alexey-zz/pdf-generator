# PDF-report generator in Rust

It is built in Rust using the following crates:

1. **Rocket** framework as HTTP Server
1. **Handlebars-rust** as template engine
1. **wkhtmltopdf** command line tool to generate PDF (not a crate)
1. Utilities: log, rust-ini, uuid

Application workflow is an synchronous call over HTTP and looks like this:

```text
HTTP Route -> Service -> Template Engine -> wkhtmltopdf - PDF
```

See report template in templates/book-order-report.html.

Sample JSON request looks like this:

```json
{
  "template_name": "book-order-report",
  "user_params": {
    "customer_name": "Frank Smith",
    "address": "Address: Frankfurt am Main, Mainzer str. 100",
    "ordered_books": [
      {
        "book_name": "Getting Things Done: The Art of Stress-Free Productivity. Authors: David Allen",
        "amount": 9.51
      },
      {
        "book_name": "Funky Business - Talent Makes Capital Dance. Authors: Ridderstråle, Nordström",
        "amount": 14.99
      },
      {
        "book_name": "The Rust Programming Language (Manga Guide). Authors: Klabnik, Nichols",
        "amount": 23.99
      }
    ],
    "total": 48.49
  }
}
```   

See more details at this INNOQ article: https://www.innoq.com/de/blog/rust-report-generator/