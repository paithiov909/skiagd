% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/png.R
\name{as_png}
\alias{as_png}
\title{Convert picture into PNG image}
\usage{
as_png(img, props = paint())
}
\arguments{
\item{img}{A raw vector of picture.}

\item{props}{A list of painting attributes out of \code{\link[=paint]{paint()}}.
For drawing functions, this can be a list of several painting attributes;
if it is a nested list, the drawing operation is performed
sequentially using each one.}
}
\value{
A raw vector of PNG image.
}
\description{
Convert picture into PNG image
}
