% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/add-circle.R
\name{add_circle}
\alias{add_circle}
\title{Add circles}
\usage{
add_circle(img, center, radius, props = paint())
}
\arguments{
\item{img}{A raw vector of picture.}

\item{center}{A double matrix where each row is circle center.}

\item{radius}{Numerics of circle radius.}

\item{props}{A list of painting attributes out of \code{\link[=paint]{paint()}}.
For drawing functions, this can be a list of several painting attributes;
if it is a nested list, the drawing operation is performed
sequentially using each one.}
}
\value{
A raw vector of picture.
}
\description{
Add circles
}
