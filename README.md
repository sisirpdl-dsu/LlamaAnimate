Initial project setup for LlamaAnimate




# Instalation guide

## 1, clone repo

```git clone https://github.com/sisirpdl-dsu/LlamaAnimate.git```

## 2, install wsl (if you dont have it already)

```
wsl --install -d Ubuntu
wsl --update
```

## 3, move into project

```
cd ./LLamaAnimate/companion
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


## 6, enter the 'master' container

```
sudo docker exec -it master bash
```

## 7, run the rust program using cargo 

```
cargo run 
```

Now you can chat with your comanion


## 8, use docker compose to stop

```
sudo docker compose down
```
















CITATION FOR vader-sentimental:


Hutto, C.J. & Gilbert, E.E. (2014). VADER: A Parsimonious Rule-based Model for Sentiment Analysis of Social Media Text. Eighth International Conference on Weblogs and Social Media (ICWSM-14). Ann Arbor, MI, June 2014.
