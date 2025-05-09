% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/add-line.R
\name{add_line}
\alias{add_line}
\title{Add lines}
\usage{
add_line(img, from, to, props = paint())
}
\arguments{
\item{img}{A raw vector of picture.}

\item{from}{A double matrix where each row is a start point.}

\item{to}{A double matrix where each row is an end point.}

\item{props}{A list of painting attributes out of \code{\link[=paint]{paint()}}.
For drawing functions, this can be a list of several painting attributes;
if it is a nested list, the drawing operation is performed
sequentially using each one.}
}
\value{
A raw vector of picture.
}
\description{
Add lines
}
