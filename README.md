# gpx-coords-tool

For a [school project](https://github.com/projetDansLaMontagne) with teamates, we are working with gpx files of hicking trails and the user location. If the user notices something of interest (a fallen tree, a nice viewpoint, ...) he can add a marker on the map. The goal of those tools are:

 - [x] To find common coordinates between the gpx files in order to add the markers on each needed gpx files.
- [x] To transform a gpx file into a json file.
- [x] To calculate the distance between two Coordinates.
  Example:
  ```
  $ calc_distance_from_json_point '{"lat": 45.0, "lon": 6.0}' '{"lat": 45.0, "lon": 6.0}'
  >> 0.0
  ```
- [ ] To find the closest point of the user location on the gpx file in order to add the marker on the closest gpx file. 1/2
  - [x] Function that finds the x closest points of a given point on a gpx file.
  - [ ] Function that adds the marker between the 2 closest points of a given point on a gpx file.
