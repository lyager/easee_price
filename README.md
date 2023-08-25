## Easee KWH Price poster

Based on the current KwH price in east Denmark, post the price to an [Easee](https://easee.com/) elecric car charger.

What you'll need from Easee is set via the following enviroments variables

    - EASEE_USERNAME
    - EASEE_PASSWORD
    - EASEE_SITE_ID

## Try it out

A docker image is available

    docker run -e EASEE_USERNAME -e EASEE_PASSWORD -e EASEE_SITE_ID lyager/easee_price:lates

## References and notes

### Getting example data

http 'https://api.energidataservice.dk/dataset/Elspotprices?end=2023-04-16&filter={PriceArea:[DK2]}&limit=23' 
