import 'package:firebase_auth/firebase_auth.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:timeline/modules/core/services/authentication_service.dart';
import 'package:timeline/modules/core/widgets/layout.dart';
import 'package:timeline/modules/login/views/splash_view.dart';

class HomeView extends StatelessWidget {
  const HomeView({Key? key}) : super(key: key);

  _logout(BuildContext context) async {
    try {
      await Provider.of<AuthenticationService>(context, listen: false)
          .signOut();

      Navigator.of(context).pushReplacement(
          MaterialPageRoute(builder: (_) => const SplashView()));
    } catch (e) {
      ScaffoldMessenger.of(context)
          .showSnackBar(const SnackBar(content: Text('Logout failed')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Layout(
      actions: [
        TextButton(
            onPressed: () => _logout(context), child: const Text('Logout'))
      ],
      body: const Center(
        child: Text('Home view'),
      ),
    );
  }
}
