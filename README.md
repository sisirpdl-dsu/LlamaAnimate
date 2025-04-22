Initial project setup for LlamaAnimate




# Instalation guide

## 1, Enter WSL (if you don't have it, install it using the command below)
```
wsl --install -d Ubuntu
wsl --update
```
Enter using this command
```
wsl
```
## 2, clone repo while in WSL

```git clone https://github.com/sisirpdl-dsu/LlamaAnimate.git```

## 3, move into project

```
cd ./LlamaAnimate/companion
```

## 4, install prerequisites using prereq.sh

```
sudo bash prereq.sh
```

if this fails exit wsl and run ```wsl --update``` again



## 5, use docker compose to run 

```
sudo docker compose up
```


## 6, enter a new tab, then enter the 'master' container

```
sudo docker exec -it master bash
```

## 7, run the rust program using cargo in the new tab

```
cargo run 
```

Now you can chat with your comanion, just type in the window once the tab has cleared. If you want to see how the program is thinking in the background, look at the original tab.

## Once done, quit out of the chat being using the command ctrl+c, then typing exit
## In the original window, press ctrl+c to stop the progrom, then use docker compose to stop the container

```
sudo docker compose down
```
















CITATION FOR vader-sentimental:


Hutto, C.J. & Gilbert, E.E. (2014). VADER: A Parsimonious Rule-based Model for Sentiment Analysis of Social Media Text. Eighth International Conference on Weblogs and Social Media (ICWSM-14). Ann Arbor, MI, June 2014.
