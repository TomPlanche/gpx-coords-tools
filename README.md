# gpx-coords-tool

For a [school project](https://github.com/projetDansLaMontagne) with teamates, we are working with gpx files of hicking trails and the user location. If the user notices something of interest (a fallen tree, a nice viewpoint, ...) he can add a marker on the map. The goal of those tools are:

- [x] `calc_distance_from_json_point`

  Calculate the distance between two Coordinates.
  Example:
  ```
  $ calc_distance_from_json_point '{"lat": 45.0, "lon": 6.0}' '{"lat": 45.0, "lon": 6.0}'
  >> 0.0
  ```

- [x] `comparator`

  Find common coordinates between the gpx files and return a json file with the common coordinates.
    Example:
    ```
    $ comparator
    >> Comparing file_x.gpx and file_y.gpx...
    >> ...
    >> Comparing file_xn.gpx and file_yn.gpx...
    >> Done, look for the 'final.json' file in the 'output' folder :)
    ```

- [ ] `find_closest_points`

  Find the x closest points of a given point on a gpx file.
  
  TODO: Determinate the binary arguments and implement the function.

  Example:
  ```
  $ find_closest_points file.gpx '{"lat": 45.0, "lon": 6.0}' 2
  >> [{"lat": 45.0, "lon": 6.0}, {"lat": 45.0, "lon": 6.0}]
  ```
  
- [x] `gpx_to_json`

    Transform all the gpx files in the `assets` folder into json files in the `output` folder.
    Example:
    ```
    $ gpx_to_json file.gpx
    >> Successfully saved to: output/file_x.json
    >> ...
    >> Successfully saved to: output/file_n.json
    ```

- [x] `file_utils`

    Contains the functions used by the other tools to read and write files (gpx and json).

- [x] `reader`

    Contains the functions used by the other tools to read gpx files
  - Find common coordinates between two files from the 'final.json' file.
  - Convert the list of common indexes into a list of common coordinates.

- [x] `tiles_to_json`

    Read the tiles from the `tiles` folder and return a json file with the tiles structure.
