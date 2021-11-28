import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

const double padding = 15;

class Layout extends StatelessWidget {
  final List<Widget> actions;
  final Widget? body;
  const Layout({Key? key, this.actions = const [], this.body})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        centerTitle: true,
        title: Hero(
          child: Text('Timeline app',
              style: Theme.of(context).textTheme.headline5),
          tag: 'bar/title',
        ),
        actions: actions,
      ),
      body: Container(
        padding: const EdgeInsets.all(padding),
        child: body,
      ),
    );
  }
}
