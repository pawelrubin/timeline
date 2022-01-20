import requests
import numpy as np
import datetime
import matplotlib.pyplot as plt


date_and_time = datetime.datetime(2021, 2, 19, 12, 0, 0)
activity = np.random.choice(["on_foot", "running", "on_bicycle", "in_vehicle"])
time_mu, time_sigma = 45, 10.0  # mean and standard deviation
location_mu, location_sigma = 0.0000, 0.0002
noise_mu, noise_sigma = 0.0000, 0.00004
x, y = 51.1081321, 17.0472596
delta_x, delta_y = np.random.normal(location_mu, location_sigma, 2)
print(date_and_time)
xs = []
ys = []
for i in range(10 ** 3):
    if np.random.uniform() < 0.1:
        delta_x, delta_y = np.random.normal(location_mu, location_sigma, 2)
    noise_x, noise_y = np.random.normal(noise_mu, noise_sigma, 2)
    x += delta_x + noise_x
    y += delta_y + noise_y
    minutes = np.random.normal(time_mu, time_sigma, 1)[0]
    time_change = datetime.timedelta(minutes=minutes)
    if np.random.uniform() < 0.001:
        date_and_time += datetime.timedelta(days=1)
    date_and_time += time_change
    if np.random.uniform() < 0.01:
        activity = np.random.choice(["on_foot", "running", "on_bicycle", "in_vehicle"])
    accuracy = 10.0

    json = [
        {
            "timestamp": date_and_time.isoformat(),
            "lat": x,
            "lng": y,
            "accuracy": 100,
            "activity": activity,
        }
    ]
    # print(json)
    ys.append(y)
    xs.append(x)

    token = "<token>"
    requests.post(
        "http://localhost:8080/geodata",
        json=json,
        headers={"Authorization": f"Bearer {token}"},
    )

plt.plot(xs, ys)
plt.show()
