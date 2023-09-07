CREATE DATABASE  IF NOT EXISTS `tournoix_test_db` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;
USE `tournoix_test_db`;
-- MySQL dump 10.13  Distrib 8.0.34, for macos13 (arm64)
--
-- Host: 127.0.0.1    Database: tournoix_test_db
-- ------------------------------------------------------
-- Server version	8.1.0

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `__diesel_schema_migrations`
--

DROP TABLE IF EXISTS `__diesel_schema_migrations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `__diesel_schema_migrations` (
  `version` varchar(50) NOT NULL,
  `run_on` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `__diesel_schema_migrations`
--

LOCK TABLES `__diesel_schema_migrations` WRITE;
/*!40000 ALTER TABLE `__diesel_schema_migrations` DISABLE KEYS */;
INSERT INTO `__diesel_schema_migrations` VALUES ('20230828085105','2023-09-01 12:45:28'),('20230828092404','2023-09-01 12:45:28'),('20230828092405','2023-09-01 12:45:28'),('20230828092420','2023-09-01 12:45:28'),('20230828092425','2023-09-01 12:45:28'),('20230828092438','2023-09-01 12:45:28'),('20230828092442','2023-09-01 12:45:28'),('20230829074038','2023-09-01 12:45:28');
/*!40000 ALTER TABLE `__diesel_schema_migrations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `bets`
--

DROP TABLE IF EXISTS `bets`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `bets` (
  `id` int NOT NULL AUTO_INCREMENT,
  `fk_games` int NOT NULL,
  `fk_teams` int NOT NULL,
  `fk_nuts` int NOT NULL,
  `nb_nut` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `bets_games` (`fk_games`),
  KEY `bets_teams` (`fk_teams`),
  KEY `bets_nuts` (`fk_nuts`),
  CONSTRAINT `bets_games` FOREIGN KEY (`fk_games`) REFERENCES `games` (`id`),
  CONSTRAINT `bets_nuts` FOREIGN KEY (`fk_nuts`) REFERENCES `nuts` (`id`),
  CONSTRAINT `bets_teams` FOREIGN KEY (`fk_teams`) REFERENCES `teams` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `bets`
--

LOCK TABLES `bets` WRITE;
/*!40000 ALTER TABLE `bets` DISABLE KEYS */;
/*!40000 ALTER TABLE `bets` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `games`
--

DROP TABLE IF EXISTS `games`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `games` (
  `id` int NOT NULL AUTO_INCREMENT,
  `fk_team1` int NOT NULL,
  `fk_team2` int NOT NULL,
  `score1` int NOT NULL,
  `score2` int NOT NULL,
  `phase` int NOT NULL,
  `place` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `matchs_team1` (`fk_team1`),
  KEY `matchs_team2` (`fk_team2`),
  CONSTRAINT `matchs_team1` FOREIGN KEY (`fk_team1`) REFERENCES `teams` (`id`),
  CONSTRAINT `matchs_team2` FOREIGN KEY (`fk_team2`) REFERENCES `teams` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `games`
--

LOCK TABLES `games` WRITE;
/*!40000 ALTER TABLE `games` DISABLE KEYS */;
/*!40000 ALTER TABLE `games` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `nuts`
--

DROP TABLE IF EXISTS `nuts`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `nuts` (
  `id` int NOT NULL AUTO_INCREMENT,
  `fk_users` int NOT NULL,
  `fk_tournaments` int NOT NULL,
  `stock` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `nuts_users` (`fk_users`),
  KEY `nuts_tournaments` (`fk_tournaments`),
  CONSTRAINT `nuts_tournaments` FOREIGN KEY (`fk_tournaments`) REFERENCES `tournaments` (`id`),
  CONSTRAINT `nuts_users` FOREIGN KEY (`fk_users`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `nuts`
--

LOCK TABLES `nuts` WRITE;
/*!40000 ALTER TABLE `nuts` DISABLE KEYS */;
/*!40000 ALTER TABLE `nuts` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `subscriptions`
--

DROP TABLE IF EXISTS `subscriptions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `subscriptions` (
  `id` int NOT NULL AUTO_INCREMENT,
  `fk_users` int NOT NULL,
  `fk_tournaments` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `subscriptions_users` (`fk_users`),
  KEY `subscriptions_tournaments` (`fk_tournaments`),
  CONSTRAINT `subscriptions_tournaments` FOREIGN KEY (`fk_tournaments`) REFERENCES `tournaments` (`id`),
  CONSTRAINT `subscriptions_users` FOREIGN KEY (`fk_users`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `subscriptions`
--

LOCK TABLES `subscriptions` WRITE;
/*!40000 ALTER TABLE `subscriptions` DISABLE KEYS */;
/*!40000 ALTER TABLE `subscriptions` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `teams`
--

DROP TABLE IF EXISTS `teams`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `teams` (
  `id` int NOT NULL AUTO_INCREMENT,
  `fk_tournaments` int NOT NULL,
  `name` varchar(255) NOT NULL,
  `group` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `teams_tournaments` (`fk_tournaments`),
  CONSTRAINT `teams_tournaments` FOREIGN KEY (`fk_tournaments`) REFERENCES `tournaments` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `teams`
--

LOCK TABLES `teams` WRITE;
/*!40000 ALTER TABLE `teams` DISABLE KEYS */;
/*!40000 ALTER TABLE `teams` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `tokens`
--

DROP TABLE IF EXISTS `tokens`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `tokens` (
  `token` varchar(255) NOT NULL,
  `fk_users` int NOT NULL,
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `expiration_date` datetime NOT NULL,
  PRIMARY KEY (`token`),
  KEY `fk_users` (`fk_users`),
  CONSTRAINT `tokens_ibfk_1` FOREIGN KEY (`fk_users`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tokens`
--

LOCK TABLES `tokens` WRITE;
/*!40000 ALTER TABLE `tokens` DISABLE KEYS */;
INSERT INTO `tokens` VALUES ('8410ef73-9581-4b75-90cc-2c3f4e126019',8,'2023-09-07 15:05:35','2023-09-07 20:05:36');
/*!40000 ALTER TABLE `tokens` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `tournaments`
--

DROP TABLE IF EXISTS `tournaments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `tournaments` (
  `id` int NOT NULL AUTO_INCREMENT,
  `fk_users` int NOT NULL,
  `name` varchar(255) NOT NULL,
  `description` varchar(255) NOT NULL,
  `date` datetime DEFAULT NULL,
  `location` varchar(255) DEFAULT NULL,
  `phase` int NOT NULL,
  `size_group` int DEFAULT NULL,
  `code` varchar(16) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `tournaments_users` (`fk_users`),
  CONSTRAINT `tournaments_users` FOREIGN KEY (`fk_users`) REFERENCES `users` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `tournaments`
--

LOCK TABLES `tournaments` WRITE;
/*!40000 ALTER TABLE `tournaments` DISABLE KEYS */;
INSERT INTO `tournaments` VALUES (3,2,'Tournoix Incr','Incr','2001-09-23 00:00:00','Yverson',1,4,'OTxAPUitblNSpnFa'),(4,2,'Tournoix De Merde','Incr','2001-09-23 00:00:00','Yverson',1,4,'eDwI9kIYnZePRR3I');
/*!40000 ALTER TABLE `tournaments` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `users` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES (1,'Rhyan Robertson','rhyan.robertson@heig-vd.ch','$argon2id$v=19$m=65536,t=2,p=4$0WSA1UqqEgtV8EQE12/VANTMMtournoix_test_dbElsN0CDWDIbueymo$Lg378B+iVtVjyzfs0GzrmLx+mHsZPUwCcgFjkVTXnzs'),(2,'Rhyan Robertson 2','rhyan.robertson@heigvd.ch','$argon2id$v=19$m=65536,t=2,p=4$UNjCB2xno0L1xrPL0BJGpl4jDW4fHqwqeYeNThIvKDA$b5fnn8MXzZ15OLvpFtrTjJHmpgvjTAxXKZI2NMYob5U'),(8,'John Doe','john.doe@tournoix.com','$argon2id$v=19$m=65536,t=2,p=4$Cz3RWQWsIZ7kUJwIdQZ/rFf27aOzcvgGx6iu/odCo/s$LM8Gd7jW+3nys04Onr8oF7kkqDF0q5IKy4jtGLjJ/Zs');
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-09-07 18:36:50
