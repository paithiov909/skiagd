% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/path-utils.R
\name{path_interpolate}
\alias{path_interpolate}
\title{Interpolate between two SVG paths}
\usage{
path_interpolate(t, first, second)
}
\arguments{
\item{t}{A numeric vector of weights in range \verb{(0, 1)}.
If it is inbetween or outside the range, the path is interpolated.}

\item{first}{A string scalar of SVG notation.}

\item{second}{A string scalar of SVG notation.}
}
\value{
A character vector of SVG notations interpolated.
}
\description{
Interpolates between two SVG paths of equal length.
If the paths are not interpolatable, an error is thrown.
}
\seealso{
Other path-utils: 
\code{\link{path_bounds}()},
\code{\link{path_transform}()}
}
\concept{path-utils}
