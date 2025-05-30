% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/add-textblob.R
\name{add_text}
\alias{add_text}
\alias{text_layout_horizontal}
\alias{text_layout_vertical}
\alias{text_width}
\title{Add text}
\usage{
add_text(img, text, point = NULL, ..., props = paint())

text_layout_horizontal(text, props = paint())

text_layout_vertical(text, props = paint())

text_width(text, props = paint())
}
\arguments{
\item{img}{A raw vector of picture.}

\item{text}{Characters to be drawn.}

\item{point}{\code{NULL} or a double matrix where each row is the point
at which each character in \code{text} is drawn.
For example, if \code{text} is a character vector of 5 and 3 length strings,
\code{point} must contain 8 points.
If \code{NULL}, \code{text} is drawn at \code{c(0, props[["fontsize"]])} naturally.}

\item{...}{For some drawing functions, you can specify \code{width} and \code{color} as named arguments.
\itemize{
\item \code{width} must be a numeric vector of stroke widths for each shape.
\item \code{color} must be an integer matrix where each \emph{\strong{column}} is an RGBA color for each shape.
}

If they are not provided as named arguments, they will be taken from \code{props}.
If the function does not matter them,  \code{...} is simply ignored.}

\item{props}{A list of painting attributes out of \code{\link[=paint]{paint()}}.}
}
\value{
For \code{add_text()}, a raw vector of picture.
}
\description{
Draws text as textblobs.

The return value is often a large object
because the specified font is embedded in the returned picture.
Note that you should almost always \code{\link[=freeze]{freeze()}} the picture after drawing text.

You can use \code{text_layout_horizontal()} and \code{text_layout_vertical()}
to create a \code{point} matrix
and \code{text_width()} to get widths of textblobs.
}
\details{
Since textblobs do not have font fallback mechanism,
characters out of the specified font are not drawn correctly.
}
