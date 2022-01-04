This was an attempt at automatically retrieving the urls of Youtube videos for the FFXIV soundtrack
(which I own), uploaded by other people (who probably weren’t allowed to do that), in order to
import them in bulk in the media player of Letsrole.com because it only supports Youtube urls.

In the end, the output format shows that the actual urls are somewhat obscured behind a CDS at
Google, which may be easy to resolve but that’s as far as I’m willing to go to avoid potential
issues with my Google account.

It was not entirely fruitless, with actual use of Sled, a pure-rust database, and Governor, a very
good throttling library (among other things).
