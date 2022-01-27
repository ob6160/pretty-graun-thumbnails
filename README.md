# pretty-grauniad-thumbnails
Created for the Guardian Fastly Hack day. I aimed to improve the rendering of our social media post sharing thumbnail images. Namely, I wanted to add some details about the article content to the image itself to make it clearer to users what they are clicking on.

Here's what they look like now:
![the current guardian thumbnail format, with the logo on the bottom right](https://i.guim.co.uk/img/media/39a732eb180224e65b8fc717c7e386630f0872d0/0_3_1360_816/master/1360.jpg?width=1200&height=630&quality=85&auto=format&fit=crop&overlay-align=bottom%2Cleft&overlay-width=100p&overlay-base64=L2ltZy9zdGF0aWMvb3ZlcmxheXMvdGctZGVmYXVsdC5wbmc&enable=upscale&s=6bd6147c5504ff426afb5f68fdb1cb26)

And the result of the hack:
![the output from this program, adding a newer version of the logo as well as the article title and short description](https://subtly-ancient-mayfly.edgecompute.app/thumbnail.jpg?article=https://www.theguardian.com/politics/2022/jan/26/tory-mps-poised-to-send-letters-of-no-confidence-in-pm-after-partygate-report)

This runs on Fastly’s **Compute@Edge**

Useful commands:
- `fastly compute serve --skip-verification --watch`
- `fastly compute deploy`
