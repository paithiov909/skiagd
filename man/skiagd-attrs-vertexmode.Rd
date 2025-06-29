% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/000-wrappers.R
\docType{data}
\name{VertexMode}
\alias{VertexMode}
\title{VertexMode (0-2)}
\format{
An object of class \code{VertexMode__bundle} (inherits from \code{savvy_skiagd__sealed}) of length 3.
}
\usage{
VertexMode
}
\description{
\code{VertexMode} determines how vertices are drawn.
This is for \code{\link[=add_vertices]{add_vertices()}} only. Not used in other functions.
}
\details{
The following \code{VertexMode} are available:
\itemize{
\item \code{Triangles}
\item \code{TriangleStrip}
\item \code{TriangleFan}
}
}
\seealso{
\href{https://rust-skia.github.io/doc/skia_safe/vertices/enum.VertexMode.html}{VertexMode in skia_safe::vertices - Rust}

Other paint-attributes: 
\code{\link{BlendMode}},
\code{\link{BlurStyle}},
\code{\link{Cap}},
\code{\link{FillType}},
\code{\link{FontStyle}},
\code{\link{ImageFilter}},
\code{\link{Join}},
\code{\link{PathEffect}},
\code{\link{PointMode}},
\code{\link{Shader}},
\code{\link{Style}}
}
\concept{paint-attributes}
\keyword{datasets}
