#!/bin/bash

Interval=$(grep 'interval_minutes' config.toml | cut -d '=' -f 2)

./anytype-notify

sleep $(($Interval * 60))
