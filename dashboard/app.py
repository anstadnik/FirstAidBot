import dash_bootstrap_components as dbc
import pandas as pd
import plotly.express as px
import redis
from dash import Dash, dcc, html
from dash_bootstrap_templates import load_figure_template
from flask_caching import Cache
from plotly.graph_objs import Figure

# cache = Cache(app.server, config={
#     # try 'filesystem' if you don't want to setup redis
#     'CACHE_TYPE': 'redis',
#     'CACHE_REDIS_URL': os.environ.get('REDIS_URL', '')
# })


def get_df():
    r = redis.Redis(host="localhost", port=6379, db=0)
    keys = [
        k
        for k in map(lambda s: s.decode("utf-8"), r.keys())
        if k.startswith("user_") and k != "user_ids"
    ]

    data = []
    for k in keys:
        hkeys = r.hkeys(k)
        for hk in map(lambda s: s.decode("utf-8"), hkeys):
            v = r.hget(k, hk)
            assert v is not None
            v = v.decode("utf-8")
            data.append([int(k.removeprefix("user_")), int(hk), v])

    df = pd.DataFrame(data=data, columns=["user_id", "timestamp", "state"])
    timestamp = df["timestamp"]
    assert isinstance(timestamp, pd.Series)
    df["time"] = pd.to_datetime(timestamp, unit="ms")
    return df


def get_graphs(df: pd.DataFrame) -> list[Figure]:

    state = df["state"]

    assert isinstance(state, pd.Series)
    frequencies = (
        state.str.split(";").str[0].str.split("->").str[0].value_counts().drop("")
    )
    frequencies = frequencies.reset_index().rename(
        {"index": "name", "state": "frequency"}, axis="columns"
    )

    fig_frequencies = px.pie(
        frequencies, names="name", values="frequency", title="Frequency of each button"
    )

    users_by_day = df.resample("D", on="time")["user_id"].nunique()
    users_by_day = users_by_day.reset_index().rename(
        {"time": "date", "user_id": "n_users"}, axis="columns"
    )

    fig_n_users = px.bar(
        users_by_day, y="n_users", x="date", title="Number of unique users per day"
    )

    n_users = df["user_id"]
    assert isinstance(n_users, pd.Series)
    n_users = n_users.nunique()

    new_users_per_day = df.groupby("user_id")["time"].min().dt.date.value_counts()
    new_users_per_day = new_users_per_day.reset_index().rename(
        {"index": "date", "time": "n_new_users"}, axis="columns"
    )

    fig_new_users = px.bar(
        new_users_per_day,
        y="n_new_users",
        x="date",
        title="Number of new users per day",
    )
    return [fig_frequencies, fig_n_users, fig_new_users, n_users]


def get_layout():
    df = get_df()
    fig_frequencies, fig_n_users, fig_new_users, n_users = get_graphs(df)

    return html.Div(
        children=[
            dbc.Row(
                dbc.Col(
                    [
                        html.H1("FirstAidRobot"),
                        "Коли не їде швидка",
                    ]
                ),
                style={"margin-left": "7px", "margin-top": "7px"},
            ),
            html.Hr(),
            dbc.Row(
                [
                    dbc.Col(
                        dcc.Graph(id="fig_frequencies", figure=fig_frequencies),
                        width="auto",
                    ),
                    dbc.Col(
                        html.H1(
                            [n_users, html.Br(), "users"],
                            style={
                                "textAlign": "center",
                                "color": "red",
                                "fontSize": 50,
                            },
                        ),
                        width="auto",
                    ),
                ],
                justify="center",
                align="center",
            ),
            html.Hr(),
            dbc.Row(dbc.Col(dcc.Graph(id="fig_n_users", figure=fig_n_users))),
            html.Hr(),
            dbc.Row(dbc.Col(dcc.Graph(id="fig_new_users", figure=fig_new_users))),
        ]
    )
    # iv(
    #             [
    #                 html.H1("FirstAidRobot Dashboard"),
    #             ]
    #         ),
    #         html.Div(
    #             """
    #             Коли не їде швидка
    #             """
    #         ),
    #     ]
    # )


theme = dbc.themes.LUX
load_figure_template("LUX")
app = Dash(__name__, external_stylesheets=[theme])

app.layout = get_layout()


def main():
    # TODO: Use it: https://dash.plotly.com/performance
    app.run_server(debug=True)


if __name__ == "__main__":
    main()
