library(tidyverse)
library(patchwork)


# Data from solver
dat <- readr::read_csv(file.path("outputs", "output.dat"), col_names = c("time","S", "I","R", "Re"))

nrow(dat)

# My themes
theme_set(theme_classic(base_size = 14))

# Looks at each compartment
fig_a <- dat |>
gather(compartment, value, -time) |>
ggplot(aes(time/365, value, color = compartment))+
geom_line()+
theme(legend.position = "top")

# Look at the Rt over time in a very crude way
fig_b <- dat |>
mutate(Re = S/(S+I+R)*(2.5)) |>
ggplot(aes(time/365, Re))+
geom_line()+
theme(legend.position = "top")+
geom_hline(yintercept = 1, lwd = 1.1, lty = 2, color = "tomato" )+
scale_y_continuous(name = "Effective Reproduction Number", limits = c(0,NA), expand = c(0,0))

# Make something to look at
png("assets/graph.png")
fig_a /fig_b
dev.off()

# trying again
