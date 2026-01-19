library(jsonlite)
opt_data <- fromJSON("optimized_results.json")
# deg_space_data <- fromJSON("deg_space_results_perm.json")
# sorting_method input_data_type given_n durations
#
# pub enum SortingMethod {
#    Merge,
#   Insertion,
#   Bubble,
# str(data)
# pub enum InputDataType {
# UniRand,
# Sorted,
# RevSorted,
# AlmostSorted,
# PipeOrgan,
#
# given_n    2 6 16 47 135 387 1101 3153 9019 10000 ...

# print(data[2, ])
# bub_uni_ns <- data[
# data$sorting_method == "Bubble" &
# data$input_data_type == "UniRand",
# ]


# print(unlist(bub_uni_ns[bub_uni_ns$given_n == 2, "durations"]))
# print("got here")

# UNIRAND
xs <- list(2, 6, 16, 47, 135, 387, 1101, 3153, 9019, 10000)
png("uniform_random.png", width = 800, height = 600)

UniRand_opt <- opt_data[opt_data$input_data_type == "UniRand", ]
UniRand_opt$average <- sapply(UniRand_opt$durations, mean)

# print("UniRand Bubble Insertion Merge ")
# print(UniRand_opt[
#   UniRand_opt$sorting_method == "Bubble" &
#     UniRand_opt$given_n == 10000,
#   "durations"
# ])
# print(UniRand_opt[
#   UniRand_opt$sorting_method == "Insertion" &
#     UniRand_opt$given_n == 10000,
#   "durations"
# ])
# print(UniRand_opt[
#   UniRand_opt$sorting_method == "Merge" &
#     UniRand_opt$given_n == 10000,
#   "durations"
# ])
#
# Sorted_opt <- opt_data[opt_data$input_data_type == "Sorted", ]
# Sorted_opt$average <- sapply(Sorted_opt$durations, mean)
#
# print("Sorted Bubble Insertion Merge ")
# print(Sorted_opt[
#   Sorted_opt$sorting_method == "Bubble" &
#     Sorted_opt$given_n == 10000,
#   "durations"
# ])
# print(Sorted_opt[
#   Sorted_opt$sorting_method == "Insertion" &
#     Sorted_opt$given_n == 10000,
#   "durations"
# ])
# print(Sorted_opt[
#   Sorted_opt$sorting_method == "Merge" &
#     Sorted_opt$given_n == 10000,
#   "durations"
# ])

y1 <- UniRand_opt[UniRand_opt$sorting_method == "Bubble", "average", ]
y2 <- UniRand_opt[UniRand_opt$sorting_method == "Insertion", "average", ]
y3 <- UniRand_opt[UniRand_opt$sorting_method == "Merge", "average", ]

plot(
  xs,
  y1,
  type = "l",
  col = "blue",
  xlab = "Length of Input",
  ylab = "Time (In nano seconds)",
  main = "Uniform Random",
)

lines(
  xs,
  y2,
  col = "red",
)
lines(
  xs,
  y3,
  col = "green",
)
grid()


y1_max <- round(y1[length(y1)])
y2_max <- round(y2[length(y2)])
y3_max <- round(y3[length(y3)])

legend("topleft",
  legend = c(
    paste("Bubble\n", y1_max, "ns"),
    paste("Insertion \n", y2_max, "ns"),
    paste("Merge \n", y3_max, "ns")
  ),
  col = c("blue", "red", "green"),
  lty = 1,
  cex = 1
)

dev.off()


################################################################

# SORTED
################################################################
png("sorted.png", width = 800, height = 600)
Sorted_opt <- opt_data[opt_data$input_data_type == "Sorted", ]
Sorted_opt$average <- sapply(Sorted_opt$durations, mean)


# Merge goes way off here

y1 <- Sorted_opt[Sorted_opt$sorting_method == "Bubble", "average", ]
y2 <- Sorted_opt[Sorted_opt$sorting_method == "Insertion", "average", ]
y3 <- Sorted_opt[Sorted_opt$sorting_method == "Merge", "average", ]

y1_max <- round(y1[length(y1)])
y2_max <- round(y2[length(y2)])
y3_max <- round(y3[length(y3)])



plot(xs,
  y3,
  type = "l",
  col = "green",
  xlab = "Length of Input",
  ylab = "Time (In nano seconds)",
  main = "Sorted",
  pch = 4
)

lines(xs, y2, col = "red")
lines(xs, y1, col = "blue")
grid()


legend("topleft",
  legend = c(
    paste("Bubble\n", y1_max, "ns"),
    paste("Insertion \n", y2_max, "ns"),
    paste("Merge \n", y3_max, "ns")
  ),
  col = c("blue", "red", "green"),
  lty = 1,
  cex = 1
)
dev.off()
##############################################################

# REVSORTED
################################################################
png("reverse_sorted.png", width = 800, height = 600)
Rev_Sorted_opt <- opt_data[opt_data$input_data_type == "RevSorted", ]
Rev_Sorted_opt$average <- sapply(Rev_Sorted_opt$durations, mean)



y1 <- Rev_Sorted_opt[Rev_Sorted_opt$sorting_method == "Bubble", "average", ]
y2 <- Rev_Sorted_opt[Rev_Sorted_opt$sorting_method == "Insertion", "average", ]
y3 <- Rev_Sorted_opt[Rev_Sorted_opt$sorting_method == "Merge", "average", ]

y1_max <- round(y1[length(y1)])
y2_max <- round(y2[length(y2)])
y3_max <- round(y3[length(y3)])



plot(xs,
  y1,
  type = "l",
  col = "blue",
  xlab = "Length of Input",
  ylab = "Time (In nano seconds)",
  main = "Reverse Sorted",
  pch = 4
)

lines(xs, y2, col = "red")
lines(xs, y3, col = "green")
grid()


legend("topleft",
  legend = c(
    paste("Bubble\n", y1_max, "ns"),
    paste("Insertion \n", y2_max, "ns"),
    paste("Merge \n", y3_max, "ns")
  ),
  col = c("blue", "red", "green"),
  lty = 1,
  cex = 1
)
dev.off()
##############################################################

# Almost Sorted
################################################################
png("almost_sorted.png", width = 800, height = 600)
Alm_Sorted_opt <- opt_data[opt_data$input_data_type == "AlmostSorted", ]
Alm_Sorted_opt$average <- sapply(Alm_Sorted_opt$durations, mean)


y1 <- Alm_Sorted_opt[Alm_Sorted_opt$sorting_method == "Bubble", "average", ]
y2 <- Alm_Sorted_opt[Alm_Sorted_opt$sorting_method == "Insertion", "average", ]
y3 <- Alm_Sorted_opt[Alm_Sorted_opt$sorting_method == "Merge", "average", ]

y1_max <- round(y1[length(y1)])
y2_max <- round(y2[length(y2)])
y3_max <- round(y3[length(y3)])



plot(xs,
  y1,
  type = "l",
  col = "blue",
  xlab = "Length of Input",
  ylab = "Time (In nano seconds)",
  main = "Almost Sorted",
  pch = 4,
)

lines(xs, y2, col = "red")
lines(xs, y3, col = "green")
grid()


legend("topleft",
  legend = c(
    paste("Bubble\n", y1_max, "ns"),
    paste("Insertion \n", y2_max, "ns"),
    paste("Merge \n", y3_max, "ns")
  ),
  col = c("blue", "red", "green"),
  lty = 1,
  cex = 1
)
dev.off()
##############################################################

# Pipe Organ
################################################################
png("pipe_organ.png", width = 800, height = 600)
Pipe_Organ_opt <- opt_data[opt_data$input_data_type == "PipeOrgan", ]
Pipe_Organ_opt$average <- sapply(Pipe_Organ_opt$durations, mean)


y1 <- Pipe_Organ_opt[Pipe_Organ_opt$sorting_method == "Bubble", "average", ]
y2 <- Pipe_Organ_opt[Pipe_Organ_opt$sorting_method == "Insertion", "average", ]
y3 <- Pipe_Organ_opt[Pipe_Organ_opt$sorting_method == "Merge", "average", ]

y1_max <- round(y1[length(y1)])
y2_max <- round(y2[length(y2)])
y3_max <- round(y3[length(y3)])



plot(xs,
  y1,
  type = "l",
  col = "blue",
  xlab = "Length of Input",
  ylab = "Time (In nano seconds)",
  main = "Pipe Organ",
  pch = 4,
)

lines(xs, y2, col = "red")
lines(xs, y3, col = "green")
grid()


legend("topleft",
  legend = c(
    paste("Bubble\n", y1_max, "ns"),
    paste("Insertion \n", y2_max, "ns"),
    paste("Merge \n", y3_max, "ns")
  ),
  col = c("blue", "red", "green"),
  lty = 1,
  cex = 1
)
dev.off()
##############################################################
