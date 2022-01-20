import datetime
import asyncio

import numpy as np
import matplotlib.pyplot as plt
import httpx

TOKEN = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjQwMTU0NmJkMWRhMzA0ZDc2NGNmZWUzYTJhZTVjZDBlNGY2ZjgyN2IiLCJ0eXAiOiJKV1QifQ.eyJuYW1lIjoiUGF3ZcWCIFJ1YmluIiwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmdvb2dsZXVzZXJjb250ZW50LmNvbS9hLS9BT2gxNEdpTzFJNE1kQnJER1U3NWpseVFyT3l2MWZFZS1Odllmb3YtNmNRaHIwdz1zOTYtYyIsImlzcyI6Imh0dHBzOi8vc2VjdXJldG9rZW4uZ29vZ2xlLmNvbS90aW1lbGluZS0zMzE3MTgiLCJhdWQiOiJ0aW1lbGluZS0zMzE3MTgiLCJhdXRoX3RpbWUiOjE2NDI0NTcxMTEsInVzZXJfaWQiOiJpdjB4NmVyZkNpaDRmeFFLYmp4NjlqZWVuWjcyIiwic3ViIjoiaXYweDZlcmZDaWg0ZnhRS2JqeDY5amVlblo3MiIsImlhdCI6MTY0MjY4NTc5OSwiZXhwIjoxNjQyNjg5Mzk5LCJlbWFpbCI6InBhd2VscnViaW4xOUBnbWFpbC5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiZmlyZWJhc2UiOnsiaWRlbnRpdGllcyI6eyJnb29nbGUuY29tIjpbIjEwMjk1MTc2MjQ2ODAyMDQ2MjU5MiJdLCJlbWFpbCI6WyJwYXdlbHJ1YmluMTlAZ21haWwuY29tIl19LCJzaWduX2luX3Byb3ZpZGVyIjoiZ29vZ2xlLmNvbSJ9fQ.y29Erw0nsyeph4UvNJHRLIxnryYhJ3ycXv1bMW39GoNE9X-qYMuYP2tFR4cRadBgyOPrfod1-Yt9YckSwKD_DeILT9ulPTzM_eFSXamDDcXhxU_HJjS7ViV3mGs5I6m1GiEQ0poiArpyFDNSbU0i6NrH9bG2v5jPVzr7I-2AB-AGVx6oTyRYVRqZABl0Ho-5ETTY8D8zoMorUUYZjMkGvVJv-ikdNyF9jTis5EP3EHBocoI8WnqBfCmV0lyuiAEJv__USyE7T9CUf6f77i2oxgCaGX3m_W8lyovKv0jLv5IGPF63KhkOtBa07CTir5-k_EAfF3NrzKRZRwQ2wsz-nw"


def sub_lists(lst, length):
    """yields sublist of given max length"""
    for i in range(0, len(lst), length):
        yield lst[i : i + length]


async def main():
    date_and_time = datetime.datetime(2021, 12, 19, 12, 0, 0)
    activity = np.random.choice(["on_foot", "running", "on_bicycle", "in_vehicle"])
    time_mu, time_sigma = 45, 10.0  # mean and standard deviation
    location_mu, location_sigma = 0.0000, 0.0002
    noise_mu, noise_sigma = 0.0000, 0.00004
    x, y = 51.1081321, 17.0472596
    delta_x, delta_y = np.random.normal(location_mu, location_sigma, 2)
    print(date_and_time)
    xs = []
    ys = []

    points = []

    for _ in range(10 ** 5):
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
            activity = np.random.choice(
                ["on_foot", "running", "on_bicycle", "in_vehicle"]
            )

        points.append(
            {
                "timestamp": date_and_time.isoformat(),
                "lat": x,
                "lng": y,
                "accuracy": 100,
                "activity": activity,
            }
        )

        ys.append(y)
        xs.append(x)

    points_batched = list(sub_lists(points, 100))

    async with httpx.AsyncClient(timeout=None) as client:
        await asyncio.gather(
            *[
                client.post(
                    "http://localhost:8080/geodata",
                    json=points_batch,
                    headers={"Authorization": f"Bearer {TOKEN}"},
                )
                for points_batch in points_batched
            ]
        )

    plt.plot(xs, ys)
    plt.show()


if __name__ == "__main__":
    asyncio.run(main())
