## Easee KWH Price poster

Based on the current KwH price in east Denmark, post the price to an [Easee](https://easee.com/) electric car charger.

What you'll need from Easee is set via the following enviroments variables

- EASEE_USERNAME
- EASEE_PASSWORD
- EASEE_SITE_ID

## Try it out

A docker image is available

    docker run -e EASEE_USERNAME -e EASEE_PASSWORD -e EASEE_SITE_ID lyager/easee_price:lates

## TODO

- [ ] Make it async
- [x] Investigate if `DataTime` to priceX conversion could fal.
- [x] The VAT posting to Easee is off/misunderstood
- [ ] Figure out how to read charges based on subscription

## Research section (mixed content and notes)

Nice [overview](https://www.ewii.dk/privat/el/nettariffer/) of what is what, when it comes to taxes and charges. As said: `transport is what is called "tarif"`.
Another, and more officiel overview [Aktuelle Tariffer](https://energinet.dk/el/elmarkedet/tariffer/aktuelle-tariffer/).

As a private consumer, you'll have to look at what is called "C prices", which again is subdivided into which part of Denmark you leave: east or west.

Unlike other powerconsumption calculaters, I'll assume that subscriptions are something that can be disregarded from the actual charging price - the assumption being that you will be paying those subscription fees anyway. So we will be focusing only on the pricing directly related to the KwH prices.




