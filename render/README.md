This is the rendering code for https://harrystern.net/halldisplay.html

This directory contains code that gathers weather and todo data from standard HTTP REST APIs, draws graphs and text displaying that data into an image buffer, and then formats the image data so that it can be directly displayed on a waveshare e-ink display. The data is saved to a file which can be served from a normal webserver like nginx.

[becker]
ripped out seccomp stuff. i got the error : cannot find -lseccomp:. would be best as a cargo feature
turned off todoist by just not calling it. another good feature flag. or if api key is missing
added example json

useful notes
https://www.weather.gov/documentation/services-web-api
station info via 
https://api.weather.gov/points/{latitude},{longitude}
grid id is office in the data. this has the x and y numbers as well.
station i got from the website https://forecast.weather.gov/MapClick.php?x=217&y=71&site=sew&zmx=&zmy=&map_x=217&map_y=70 the part in the () after the name
