# rust_runtimes_experiment

В рамках данного репозитория проверялась возможность запуска Tokio Runtime внутри других систем реализующих свои варианты Executor'ов. 
Далее будет приведен список систем в рамках которых проводится исследование. Для исследования выбраны проекты которые 
имеют достаточно неплохую документацию, активно развиваются сообществом и имеют достаточно хорошее и полное API с которым можно работать

Kompact [Research in progress]
=======

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/kompics/kompact)
[![Cargo](https://img.shields.io/crates/v/kompact.svg)](https://crates.io/crates/kompact)
[![Documentation](https://docs.rs/kompact/badge.svg)](https://docs.rs/kompact)
[![codecov](https://codecov.io/gh/kompics/kompact/branch/master/graph/badge.svg?token=NKH34R0CRC)](https://codecov.io/gh/kompics/kompact)
![Build Status](https://github.com/kompics/kompact/workflows/CI/badge.svg)
[![GitHub commits](https://badgen.net/github/commits/kompics/kompact)](https://GitHub.com/kompics/kompact/commit/)
[![GitHub stars](https://img.shields.io/github/stars/kompics/kompact?style=social&label=Star&maxAge=2592000)](https://GitHub.com/kompics/kompact/stargazers/)



Kompact is an in-development message-passing component system like [Kompics](https://kompics.github.io/docs/current/) in the Rust language, with performance and static typing in mind. It merges both Kompics' component model and the actor model as found in [Erlang](http://www.erlang.se/) or [Akka](https://akka.io/).

Kompact has [shown](https://kompics.github.io/kompicsbenches/) itself to vastly outperform many of its peers on a wide range of message-passing tasks, providing the capability of handling up to 400mio messages per second on 36 cores.

Kompact comes with its own network library built-in, providing easy connection maintenance and efficient serialisation for distributed deployments.

---
#### Мысли
Так как данный фреймворк использует свои Executor'ы то мы не можем просто выполнить код требующий Tokio Runtime. Однако 
Tokio позволяет ручками инстанцировать Runtime и в зависимости от нагрузки он может выполняться либо в рамках текущего потока 
либо инстанцировать отдельный пул потоков и запускать таски в нем. При таком подходе в системе может быть несколько компонентов котрые работают 
с Tokio Runtime все остальное может выполняться в рамках системы Kompact.

Из плсов:
- Достаточно хорошая документация с туториалами и примерами
- поддерживаются основные модели обмена сообщениями Ask/Tell/Channels aka Ports
- реализована модель распределенных вычислений
- фреймворк реализован по аналогии с уже существующей стистемой Kompics написанной на Scala есть бенчмарки и научные статьи
- компонентная модель схожая с акторами
- можно менеджить локальный стейт
- из коробки есть конфигурация и логирование 

На данный момент нет полного понимания как структурировать приложение, но выглядит многообещающе.

---


Bastion [Research not finished]
=======

Highly-available Distributed Fault-tolerant Runtime

Bastion is a highly-available, fault-tolerant runtime system with dynamic, dispatch-oriented, lightweight process model. It supplies actor-model-like concurrency with a lightweight process implementation and utilizes all of the system resources efficiently guaranteeing of at-most-once message delivery.

[![Cargo](https://img.shields.io/crates/v/bastion.svg)](https://crates.io/crates/bastion)
[![Documentation](https://docs.rs/bastion/badge.svg)](https://docs.rs/bastion)
[![codecov](https://codecov.io/gh/bastion-rs/bastion/branch/master/graph/badge.svg?token=NKH34R0CRC)](https://codecov.io/gh/kompics/kompact)
[![GitHub commits](https://badgen.net/github/commits/bastion-rs/bastion)](https://GitHub.com/bastion-rs/bastion/commit/)
[![GitHub stars](https://img.shields.io/github/stars/bastion-rs/bastion?style=social&label=Star&maxAge=2592000)](https://GitHub.com/bastion-rs/bastion/stargazers/)

---
#### Мысли
В принципе логика запуска кода требующего Tokio Runtimeаналогично тому как это работает в Kompact
По сути все сводится к тому что мы инстанцируем Tokio Runtime и в рамках него вызываем нужный нам код.

На данный момент не понятно как структурировать приложение в рамках данного фреймворка. Есть вопросы по
управлению состоянием в системе. Подерживается Формат обмена сообщениями Ask/Tell/Broadcast но подход к обработке сообщений
выглядит немного странным и нетривиальным. 

Остается много вопросов и пока мало ответов

---


Riker [Research in progress]
=======
[![Build status](https://github.com/riker-rs/riker/workflows/Build%20and%20run%20tests/badge.svg)](https://github.com/riker-rs/riker/actions?query=workflow%3A%22Build+and+run+tests%22)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Cargo](https://img.shields.io/crates/v/riker.svg)](https://crates.io/crates/riker)
[![Released API docs](https://docs.rs/riker/badge.svg)](https://docs.rs/riker)
[![GitHub commits](https://badgen.net/github/commits/riker-rs/riker)](https://GitHub.com/riker-rs/riker/commit/)
[![GitHub stars](https://img.shields.io/github/stars/riker-rs/riker?style=social&label=Star&maxAge=2592000)](https://GitHub.com/riker-rs/riker/stargazers/)


## Overview 

Riker is a framework for building modern, concurrent and resilient systems using the Rust language. Riker aims to make working with state and behavior in concurrent systems as easy and scalable as possible. The Actor Model has been chosen to realize this because of the familiar and inherent simplicity it provides while also providing strong guarantees that are easy to reason about. The Actor Model also provides a firm foundation for resilient systems through the use of the actor hierarchy and actor supervision.

---
#### Мысли


---
