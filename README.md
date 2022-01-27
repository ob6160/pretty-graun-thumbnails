# pretty-grauniad-thumbnails
Created for the Guardian Fastly Hack day. I aimed to improve the rendering of our social media post sharing thumbnail images. Namely, I wanted to add some details about the article content to the image itself to make it clearer to users what they are clicking on.

[Project link](https://subtly-ancient-mayfly.edgecompute.app/)

---

The project runs on Fastly’s **Compute@Edge**

Useful commands:
- `fastly compute serve --skip-verification --watch`
- `fastly compute deploy`

It also uses the excellent `image` and `imageproc` libraries for rust to do the image manipulation.

The content is served through the Guardians' CAPI (content api) infra.

## Here's what they look like now:

<img src="https://i.guim.co.uk/img/media/39a732eb180224e65b8fc717c7e386630f0872d0/0_3_1360_816/master/1360.jpg?width=1200&height=630&quality=85&auto=format&fit=crop&overlay-align=bottom%2Cleft&overlay-width=100p&overlay-base64=L2ltZy9zdGF0aWMvb3ZlcmxheXMvdGctZGVmYXVsdC5wbmc&enable=upscale&s=6bd6147c5504ff426afb5f68fdb1cb26" alt="the current guardian thumbnail format, with the logo on the bottom right" width="300px">

## And the result of the hack:

**Top alignment**

<img src="https://subtly-ancient-mayfly.edgecompute.app/thumbnail.jpg?article=https://www.theguardian.com/politics/2022/jan/26/tory-mps-poised-to-send-letters-of-no-confidence-in-pm-after-partygate-report" alt="the output from this program, adding a newer version of the logo as well as the article title and short description" width="300px">

**Bottom alignment**

<img src="https://subtly-ancient-mayfly.edgecompute.app/thumbnail.jpg?article=https://www.theguardian.com/politics/2022/jan/26/tory-mps-poised-to-send-letters-of-no-confidence-in-pm-after-partygate-report&align=bottom" alt="the output from this program, adding a newer version of the logo as well as the article title and short description" width="300px">

**Dark mode**

<img src="https://subtly-ancient-mayfly.edgecompute.app/thumbnail.jpg?lightMode=false&article=https://m.code.dev-theguardian.com/artanddesign/2021/oct/27/two-lovers-kiss-behind-a-tree-clifford-prince-kings-best-photograph" alt="the output from this program, adding a newer version of the logo as well as the article title and short description" width="300px">


