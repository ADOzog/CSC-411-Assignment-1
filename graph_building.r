library(jsonlite)
opt_data <- fromJSON("optimized_results.json")
# check this out its messing up
deg_space_data <- fromJSON("deg_space_results_perm.json")
# sorting_method input_data_type given_n durations
#
# pub enum SortingMethod {
#    Merge,
#   Insertion,
#   Bubble,
str(data)
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
png("UniRand.png", width = 800, height = 600)

UniRand_opt <- opt_data[opt_data$input_data_type == "UniRand", ]
UniRand_opt$average <- sapply(UniRand_opt$durations, mean)

UniRand_deg <- opt_data[deg_space_data$input_data_type == "UniRand", ]
UniRand_deg$average <- sapply(UniRand_deg$durations, mean)

y1 <- UniRand_opt[UniRand_opt$sorting_method == "Bubble", "average", ]
y2 <- UniRand_opt[UniRand_opt$sorting_method == "Insertion", "average", ]
y3 <- UniRand_opt[UniRand_opt$sorting_method == "Merge", "average", ]

plot(xs, y1, type = "l", col = "blue", xlab = "Length of Input", ylab = "Time (In nano seconds)", main = "UniRand")

lines(xs, y2, col = "red")
lines(xs, y3, col = "green")

# text(0, y1[length(y1)], labels = round(y1[length(y1)], 2), pos = 4, col = "blue")
# text(0, y2[length(y2)], labels = round(y2[length(y2)], 2), pos = 4, col = "red")
# text(0, y3[length(y3)], labels = round(y3[length(y3)], 2), pos = 4, col = "green")

# lines(xs, UniRand_deg[UniRand_deg$sorting_method == "Bubble", "average", ], col = "blue", lty = 2)
# lines(xs, UniRand_deg[UniRand_deg$sorting_method == "Insertion", "average", ], col = "red", lty = 2)
# lines(xs, UniRand_deg[UniRand_deg$sorting_method == "Merge", "average", ], col = "green", lty = 2)

y1_max <- round(y1[length(y1)])
y2_max <- round(y2[length(y2)])
y3_max <- round(y3[length(y3)])

legend("topleft",
  legend = c(paste("Bubble\n", y1_max, "ns"), paste("Insertion \n", y2_max, "ns"), paste("Merge \n", y3_max, "ns")),
  col = c("blue", "red", "green"), lty = 1, cex = 1
)

dev.off()

png("Sorted.png", width = 800, height = 600)

# SORTED
Sorted_opt <- opt_data[opt_data$input_data_type == "Sorted", ]
Sorted_opt$average <- sapply(Sorted_opt$durations, mean)

# UniRand_deg <- opt_data[deg_space_data$input_data_type == "UniRand", ]
# UniRand_deg$average <- sapply(UniRand_deg$durations, mean)

# Merge goes way off here
plot(xs, Sorted_opt[Sorted_opt$sorting_method == "Bubble", "average", ], type = "l", col = "blue", xlab = "Length of Input", ylab = "Time (In nano seconds)", main = "Sorted")

lines(xs, Sorted_opt[Sorted_opt$sorting_method == "Insertion", "average", ], col = "red")
lines(xs, Sorted_opt[Sorted_opt$sorting_method == "Merge", "average", ], col = "green")

# lines(xs, UniRand_deg[UniRand_deg$sorting_method == "Bubble", "average", ], col = "blue", lty = 2)
# lines(xs, UniRand_deg[UniRand_deg$sorting_method == "Insertion", "average", ], col = "red", lty = 2)
# lines(xs, UniRand_deg[UniRand_deg$sorting_method == "Merge", "average", ], col = "green", lty = 2)

legend("topright",
  legend = c("Bubble", "Insertion", "Merge"),
  col = c("blue", "red", "green"), lty = 1
)

# REV_SORTED

dev.off()
print("All done!")
