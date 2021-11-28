import 'package:hive/hive.dart';

part 'location_entry.g.dart';

@HiveType(typeId: 0)
class LocationEntry extends HiveObject {
  LocationEntry(
      {required this.longitude,
      required this.altitude,
      required this.latitude,
      required this.dateTime});

  @HiveField(0)
  double latitude;

  @HiveField(1)
  double longitude;

  @HiveField(2)
  double altitude;

  @HiveField(3)
  DateTime dateTime;
}
