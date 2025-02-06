# Spin QR Code Generator

This [Spin](https://github.com/fermyon/spin) component exposes an http-endpoint to generate QR codes.

The QR Code is generate if:
1. There is a URL-encoded query parameter in the path. e.g. `http://localhost?github.com%2Ffermyon%2Fspin`
2. A variable is passed to Spin `SPIN_APP_QR_URL=myurl spin up`

A query parameter will overwrite a variable.

And in a browser it will render as a QR code:

![QR code for github.com/fermyon/spin](qr.png)

## Using as a Spin component

To use this component in your Spin application:

1. add this as a template:

   ```
   spin templates install --git https://github.com/mikkelhegn/spin-qr-generator
   ```

1. add the component to an existing Spin application:

   ```
   spin add qr -t qr-generator --accept-defaults
   ```

