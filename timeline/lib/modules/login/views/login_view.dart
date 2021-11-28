import 'package:flutter/material.dart';
import 'package:timeline/modules/core/services/authentication_service.dart';
import 'package:timeline/modules/core/widgets/layout.dart';
import 'package:provider/provider.dart';
import 'package:timeline/modules/home/views/home_view.dart';

class LoginView extends StatelessWidget {
  const LoginView({Key? key}) : super(key: key);

  _login(BuildContext context) async {
    try {
      await Provider.of<AuthenticationService>(context, listen: false)
          .singInWithGoogle();

      Navigator.of(context)
          .pushReplacement(MaterialPageRoute(builder: (_) => const HomeView()));
    } catch (e) {
      ScaffoldMessenger.of(context)
          .showSnackBar(const SnackBar(content: Text('Login cancelled')));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Layout(
      key: key,
      body: Center(
        child: TextButton(
            onPressed: () => _login(context),
            child: const Text('Login in with Google')),
      ),
    );
  }
}
