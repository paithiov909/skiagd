% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/000-wrappers.R
\docType{data}
\name{PointMode}
\alias{PointMode}
\title{PointMode (0-2)}
\format{
An object of class \code{PointMode__bundle} (inherits from \code{savvy_skiagd__sealed}) of length 3.
}
\usage{
PointMode
}
\description{
\code{PointMode} determines how points are drawn.
This is for \code{\link[=add_point]{add_point()}} only. Not used in other functions.
}
\details{
The following \code{PointMode} are available:
\itemize{
\item \code{Points}: Draws each \code{point} as a point. The shape of point drawn depends on \code{props}.
\item \code{Lines}: Each pair of \code{point} draws a line segment. One line is drawn for every two points; each point is used once. If count is odd, the final point is ignored.
\item \code{Polygon}: Each adjacent pair of \code{point} draws a line segment. count minus one lines are drawn; the first and last point are used once.
}
}
\seealso{
\href{https://rust-skia.github.io/doc/skia_safe/canvas/enum.PointMode.html}{PointMode in skia_safe::canvas - Rust}

Other paint-attributes: 
\code{\link{BlendMode}},
\code{\link{BlurStyle}},
\code{\link{Cap}},
\code{\link{FillType}},
\code{\link{FontStyle}},
\code{\link{ImageFilter}},
\code{\link{Join}},
\code{\link{PathEffect}},
\code{\link{Shader}},
\code{\link{Style}},
\code{\link{VertexMode}}
}
\concept{paint-attributes}
\keyword{datasets}
