### Options in Qhull

#### General Qhull Options:
- **`-V`**: Enables verbose mode, providing extensive details about the computation process. Useful for debugging and understanding Qhull’s internal workings.
- **`-Qx`**: Generates more detailed messages regarding errors and warnings, aiding in the identification of issues during execution.
- **`-Qv`**: Lists the vertices and facets of the convex hull, useful for understanding the structure of the output.
- **`-QV`**: Summarizes all options and provides in-depth information about the computation, making it easier to interpret results.

#### qconvex Specific Options:
- **`-o`**: Outputs the vertices of the convex hull. This option is critical for verifying the correct vertices are included.
- **`-p`**: Outputs the input points, which can be useful for debugging and verification.
- **`-i`**: Outputs the incidence matrix, showing the relationship between vertices and facets.
- **`-Fv`**: Outputs each vertex once along with a list of incident facets, simplifying the analysis of the convex hull structure.
- **`-Q0`**: Disables the summary output to reduce verbosity when not needed.
- **`-Q2`**: Provides verbose output with detailed information on each facet, essential for a deeper understanding of the computation.
- **`-Qt`**: Skips the test for the upper Delaunay triangulation, potentially speeding up the computation in certain cases.
