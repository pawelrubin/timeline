import 'package:flutter/cupertino.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';

const String boxName = 'location';

class DatabaseService extends ChangeNotifier {
  late Box _locationBox;
  bool _initialized = false;

  DatabaseService() {
    _init();
  }

  Future<void> _init() async {
    if (_initialized == false) {
      _locationBox = await Hive.openBox(boxName);
      _initialized = true;
    }
  }
}
