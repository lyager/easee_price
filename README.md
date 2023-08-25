## Easee KWH Price poster

Based on the current KwH price in east Denmark, post the price to an [Easee](https://easee.com/) elecric car charger.

What you'll need from Easee is set via the following enviroments variables

- EASEE_USERNAME
- EASEE_PASSWORD
- EASEE_SITE_ID

## Try it out

A docker image is available

    docker run -e EASEE_USERNAME -e EASEE_PASSWORD -e EASEE_SITE_ID lyager/easee_price:lates
