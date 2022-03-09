# Habot - Your Friendly Habit Coach (Azure Hackathon)            

## Overview of My Submission

When this hackathon opened, I was ploughing through the excellent book [Atomic Habits](https://jamesclear.com/atomic-habits) by James Clear.

Good habits are key for us to build happy lives and, conversely, bad  habits can ruin them. But good habits are hard to build and bad habits  hard to break - seems like we need a helping hand.

The purpose of this hackathon entry was to build a helpful chatbot to coach you through building or breaking habits. Habot (your friendly  habit-bot) will keep you on track, monitor your progress and give you  helpful advice when you begin to falter - all a little "tongue in cheek" of course.

#### https://habot.azurewebsites.net/

Let's take a look at some of the key features of Habot:

### 1. Single Signon with Outlook Credentials

Using your Outlook OAuth credentials you can easily sign up with Habot, and it'll greet you by name and with a smile.

![](https://github.com/lucas-matt/habot/blob/main/imgs/x0.gif?raw=true)

### 2. Creating your first habit

The first thing you'll be asked to do once you log in is to create a habit.

Habot is intelligent, and can infer one of a number of habits from  what you say. For example, it can understand that "I want to begin going to the gym" is the same as "I want to start exercising more".

![](https://github.com/lucas-matt/habot/blob/main/imgs/x1.gif?raw=true)



Once your habit is confirmed, you'll get a cheery dance until next time.

### 3. Check-in and Progress

Each day you'll need to check in with Habot (I had hoped to use push  notifications here, but alas no spare time). When you check in, Habot  will keep track of your metrics.

Habot will also give you charts to more clearly demonstrate your  progress. It also has emotions, so will feel a little sad if you aren't  doing as well as hoped.

![](https://github.com/lucas-matt/habot/blob/main/imgs/x2.gif?raw=true)

### 4. Getting Help

If you're having a hard time keeping up with your habits, Habot has some advice for you depending on the problem that you're facing.

Habot will also show you inspirational videos to help you get back on track.

![](https://github.com/lucas-matt/habot/blob/main/imgs/x3.gif?raw=true)

## Building

### Habot Vue

This is a **Vue.js** application for the UI, with the web-chat control built in.

To deploy, within the project run (with a version arg):

```
./build.sh 0.0.1
```

### Rusty Habots

A **Rust** with Rocket backend to collect metric information and build charts.

When running the application it requires MONGO (a mongodb connection string) and DB (a db name).

To build run:

```
docker build -t notmattlucas/rusty-habots:0.0.1 .
```

Or use the usual cargo commands:

```
cargo check
cargo run
cargo build
```

### Bot

The bot itself needs to be deployed via the [**Bot Composer**](https://docs.microsoft.com/en-us/composer/introduction?tabs=v2x)