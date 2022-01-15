import 'package:firebase_auth/firebase_auth.dart';
import 'package:flutter/material.dart';
import 'package:timeline/modules/core/widgets/layout.dart';
import 'package:timeline/modules/home/views/home_view.dart';
import 'package:timeline/modules/login/views/login_view.dart';

class SplashView extends StatelessWidget {
  const SplashView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    WidgetsBinding.instance?.addPostFrameCallback((timeStamp) {
      var user = FirebaseAuth.instance.currentUser;
      Navigator.of(context).pushReplacement(MaterialPageRoute(
          builder: (_) => user == null ? const LoginView() : const HomeView()));
    });

    return const Layout(
        body: Center(
      child: CircularProgressIndicator(),
    ));
  }
}
