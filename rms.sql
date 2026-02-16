--
-- PostgreSQL database dump
--

\restrict OEgH8BqiO3MX3GGDcDefj0x4xD0sVSro4hnQZJFiN6ChZAzs9BQMXCWsPXS0rXQ

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: buildings; Type: TABLE; Schema: public; Owner: rms_user
--

CREATE TABLE public.buildings (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    name character varying(100) NOT NULL,
    landlord_id uuid,
    total_units_count integer NOT NULL,
    caretaker_id uuid
);


ALTER TABLE public.buildings OWNER TO rms_user;

--
-- Name: caretakers; Type: TABLE; Schema: public; Owner: rms_user
--

CREATE TABLE public.caretakers (
    user_id uuid NOT NULL,
    national_id character varying(20) NOT NULL,
    hired_at date DEFAULT CURRENT_DATE
);


ALTER TABLE public.caretakers OWNER TO rms_user;

--
-- Name: landlords; Type: TABLE; Schema: public; Owner: rms_user
--

CREATE TABLE public.landlords (
    user_id uuid NOT NULL,
    business_name character varying(100)
);


ALTER TABLE public.landlords OWNER TO rms_user;

--
-- Name: maintenance_requests; Type: TABLE; Schema: public; Owner: rms_user
--

CREATE TABLE public.maintenance_requests (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    tenant_id uuid,
    unit_id uuid,
    issue_type character varying(50) NOT NULL,
    description text,
    priority character varying(10) DEFAULT 'medium'::character varying,
    status character varying(20) DEFAULT 'pending'::character varying,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.maintenance_requests OWNER TO rms_user;

--
-- Name: rent_payments; Type: TABLE; Schema: public; Owner: rms_user
--

CREATE TABLE public.rent_payments (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    tenant_id uuid,
    amount numeric(12,2) NOT NULL,
    payment_method character varying(20),
    receipt_number integer NOT NULL,
    payment_date timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.rent_payments OWNER TO rms_user;

--
-- Name: rent_payments_receipt_number_seq; Type: SEQUENCE; Schema: public; Owner: rms_user
--

CREATE SEQUENCE public.rent_payments_receipt_number_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.rent_payments_receipt_number_seq OWNER TO rms_user;

--
-- Name: rent_payments_receipt_number_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rms_user
--

ALTER SEQUENCE public.rent_payments_receipt_number_seq OWNED BY public.rent_payments.receipt_number;


--
-- Name: tenants; Type: TABLE; Schema: public; Owner: rms_user
--

CREATE TABLE public.tenants (
    user_id uuid NOT NULL,
    payment_status character varying(20) DEFAULT 'up-to-date'::character varying
);


ALTER TABLE public.tenants OWNER TO rms_user;

--
-- Name: units; Type: TABLE; Schema: public; Owner: rms_user
--

CREATE TABLE public.units (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    building_id uuid,
    unit_number integer NOT NULL,
    tenant_id uuid,
    is_occupied boolean DEFAULT false
);


ALTER TABLE public.units OWNER TO rms_user;

--
-- Name: users; Type: TABLE; Schema: public; Owner: rms_user
--

CREATE TABLE public.users (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    role character varying(20) NOT NULL,
    first_name character varying(50) NOT NULL,
    last_name character varying(50) NOT NULL,
    phone_number character varying(15),
    email character varying(255) NOT NULL,
    password_hash text NOT NULL
);


ALTER TABLE public.users OWNER TO rms_user;

--
-- Name: vacation_notices; Type: TABLE; Schema: public; Owner: rms_user
--

CREATE TABLE public.vacation_notices (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    tenant_id uuid,
    notice_date date NOT NULL,
    submitted_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    status character varying(20) DEFAULT 'pending'::character varying
);


ALTER TABLE public.vacation_notices OWNER TO rms_user;

--
-- Name: rent_payments receipt_number; Type: DEFAULT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.rent_payments ALTER COLUMN receipt_number SET DEFAULT nextval('public.rent_payments_receipt_number_seq'::regclass);


--
-- Data for Name: buildings; Type: TABLE DATA; Schema: public; Owner: rms_user
--

COPY public.buildings (id, name, landlord_id, total_units_count, caretaker_id) FROM stdin;
cce58641-86d3-42d7-8daf-40e13809844d	Aparmtment2	c1e95946-137b-4b6a-a81e-c6fb5789adb0	50	\N
5cc241d3-1c48-4731-aa65-7d92db3e88bf	Aparmtment4	c1e95946-137b-4b6a-a81e-c6fb5789adb0	15	\N
2e210c58-70a5-4134-b5a8-405fb307656a	Aparmtment3	c1e95946-137b-4b6a-a81e-c6fb5789adb0	20	096a506b-068d-49e2-ac3e-0fc3767db658
1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	Aparmtment1	c1e95946-137b-4b6a-a81e-c6fb5789adb0	100	d5958480-f079-4f28-9a38-cf8983c4155d
\.


--
-- Data for Name: caretakers; Type: TABLE DATA; Schema: public; Owner: rms_user
--

COPY public.caretakers (user_id, national_id, hired_at) FROM stdin;
096a506b-068d-49e2-ac3e-0fc3767db658	TEMP-096A506B	2026-02-06
d5958480-f079-4f28-9a38-cf8983c4155d	12345678	2026-01-05
\.


--
-- Data for Name: landlords; Type: TABLE DATA; Schema: public; Owner: rms_user
--

COPY public.landlords (user_id, business_name) FROM stdin;
c1e95946-137b-4b6a-a81e-c6fb5789adb0	Apartment Agency
\.


--
-- Data for Name: maintenance_requests; Type: TABLE DATA; Schema: public; Owner: rms_user
--

COPY public.maintenance_requests (id, tenant_id, unit_id, issue_type, description, priority, status, created_at) FROM stdin;
4be56d93-9b53-4f95-b5ef-f1788711a15a	a5436461-f586-43fe-879a-a8197a088718	890ee4b0-7222-47ee-bf17-ae24c4bf391f	general	Broken+cupboard	medium	pending	2026-02-07 07:53:18.505256
\.


--
-- Data for Name: rent_payments; Type: TABLE DATA; Schema: public; Owner: rms_user
--

COPY public.rent_payments (id, tenant_id, amount, payment_method, receipt_number, payment_date) FROM stdin;
\.


--
-- Data for Name: tenants; Type: TABLE DATA; Schema: public; Owner: rms_user
--

COPY public.tenants (user_id, payment_status) FROM stdin;
a5436461-f586-43fe-879a-a8197a088718	up-to-date
\.


--
-- Data for Name: units; Type: TABLE DATA; Schema: public; Owner: rms_user
--

COPY public.units (id, building_id, unit_number, tenant_id, is_occupied) FROM stdin;
20533125-159a-4472-8792-51b1e8924f08	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	1	\N	f
5cf7e253-4064-410f-a0fe-1cb104ef9880	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	2	\N	f
bcfe7f12-d772-440c-b817-4c9893a97ddd	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	3	\N	f
301a010f-feaa-4cc6-b37c-e401c744c3b9	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	4	\N	f
d219a9df-1f66-4497-9971-352e6b32985e	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	5	\N	f
d7e235e1-7511-49d7-885a-1171720f1fb9	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	6	\N	f
181f43a0-5a87-4be8-a538-0dbe4640c4d3	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	7	\N	f
53ecbbdf-f273-4f8c-a029-b0d8c885f6ec	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	8	\N	f
70e0d1d3-f2da-4f72-bb07-ecb3b8ba90c8	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	9	\N	f
e7181e80-6612-4c16-a240-8b1b1f82485d	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	10	\N	f
cc8366d2-7b74-4545-a629-478490fa73cd	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	11	\N	f
420d96b0-d6b0-4286-bca4-831b0877b6fb	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	12	\N	f
d9020f51-aaa9-4a0e-92c2-3766c1be6452	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	13	\N	f
61cf23f6-6abd-4713-a76f-b1c17fbe9118	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	14	\N	f
75a102ce-0bbd-4cb9-94be-49e375efb4d6	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	15	\N	f
779acd7f-4147-4c14-9e93-b0ef5ffcde54	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	16	\N	f
07ad87e1-cc46-41e5-b5f0-226cfccc5a24	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	17	\N	f
1e70cfdd-9609-45f7-a7cf-6944cdac5d91	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	18	\N	f
362acda8-9eec-4874-ba42-f1ec788e3a42	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	19	\N	f
4ca0a636-e27f-4085-9fb5-c28b4d088d3e	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	20	\N	f
6bd30c33-10be-4cf8-869f-3772ec2296af	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	21	\N	f
fe0cf30e-fa8e-4689-bc83-1bfb2aa24316	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	22	\N	f
3230c081-7bf3-4904-abb0-d8bf4610cbe4	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	23	\N	f
2d81d9f7-8b6c-4915-a6f0-71ac73a1ed51	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	24	\N	f
91f0e0bd-3de7-4cde-b89d-f4cc76a16c4b	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	25	\N	f
52f608c7-4bbc-49eb-b28b-b2c390899197	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	26	\N	f
26d167fc-68cd-45f0-ad9d-d605fb084011	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	27	\N	f
e273c119-2eb5-461a-a14e-ebdd3fca2c1b	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	28	\N	f
8007376d-94eb-41e2-b548-684da98ed8ef	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	29	\N	f
a17ba2ef-ceda-4dc2-91ae-208d10389c39	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	30	\N	f
35d6606e-6cd2-4755-9e17-493a14ef542f	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	31	\N	f
c20dbef0-69c8-4008-999a-dc1a2920bce4	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	32	\N	f
f5683a21-a951-432b-b9f7-817997117539	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	33	\N	f
5ca2bd57-ac8a-452f-9650-f1fa524bb8bf	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	34	\N	f
e83232a1-ba14-4693-8a43-9386599eafa2	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	35	\N	f
08a4ae6e-6cb9-4328-87d3-ea1a78575f1a	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	36	\N	f
3fbe9b17-4e7e-4e06-9a5a-ea557adc941e	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	37	\N	f
d5a4671a-bb77-49e6-b8c4-706203a0fe69	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	38	\N	f
9ec884f5-04ee-4beb-bcbd-c10978f6138f	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	39	\N	f
dac31a0d-eb58-4a1d-8a9f-eff582af4a02	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	40	\N	f
d660c1f6-f354-41fc-bfb3-2ab75d706d70	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	41	\N	f
32417c8d-93be-4bed-99ea-d4454399f466	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	42	\N	f
5a4aa910-7bbe-4fa5-a622-5ac5a1df5e1e	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	43	\N	f
eb6623da-1923-45ac-962d-586826ff4ca2	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	44	\N	f
b9fa66ae-cbde-484b-ac16-089afbce9bdf	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	45	\N	f
1004a2c8-c733-4397-83a5-09681d350d76	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	46	\N	f
e54f578b-218c-4aec-ab9e-d2942e913d58	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	47	\N	f
af972082-f9d3-4e96-8316-b6f0d23eedeb	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	48	\N	f
70e2b801-b94f-4dd6-a0f0-26508c9fe5b3	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	49	\N	f
99dac12d-5c0c-4554-8f53-77ca6db96cdf	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	50	\N	f
cfad0cc3-6a89-424c-9b4f-c7a1f302a3dc	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	51	\N	f
669c0573-cbec-40bc-8101-53dd78dd6a7f	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	52	\N	f
eaa62956-0754-4e1b-adb4-2832de69ecbc	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	53	\N	f
80d1d9db-2bed-41c9-90a8-51da1ff2e440	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	54	\N	f
e357f215-3476-4d3e-aad3-7f09476e415c	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	55	\N	f
1e8e230a-c0fa-449a-8ce7-e92b440c47c5	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	56	\N	f
961a38fb-7d3c-4c69-9780-92ee4a4c9210	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	57	\N	f
d1b547d9-8a99-49df-8199-da88d7e1c279	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	58	\N	f
abddf082-d578-47fa-9268-dca6cc7427c4	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	59	\N	f
a6db4646-c8e5-466c-9f79-fea19973bf5a	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	60	\N	f
bb91c962-a964-456f-870b-97d43b1b98cf	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	61	\N	f
b533178a-e999-4d6b-a8a6-fe1050554736	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	62	\N	f
27134852-474f-4464-907d-e1aebd8022e8	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	63	\N	f
5865ec67-e830-47c1-b59b-5134693409d7	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	64	\N	f
a13ce636-0ae9-460c-b470-1328ba767ed4	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	65	\N	f
864a0cd8-eaee-420a-babe-ea6c80ec45d6	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	66	\N	f
8f7fa200-293d-4406-ac4b-d7a550b8abfe	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	67	\N	f
09f12d83-233a-4ac3-8800-1b7030db6e60	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	68	\N	f
86ef5e04-aac4-4725-bb40-f376c8067909	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	69	\N	f
f0bc3822-a078-42bd-9015-b15d53bc12b3	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	70	\N	f
839ff26a-905a-4a43-a4a8-8544f91f3bc7	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	71	\N	f
4f39634a-8e84-44c5-b2a7-916f8f7fe0b8	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	72	\N	f
bbabd2c8-1375-4bbb-9cbf-e04547964d46	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	73	\N	f
48953d49-a9ef-4cd8-85d6-e2ee4907a372	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	74	\N	f
ebe5b13c-aea4-4a8f-85f6-8a43f3761fba	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	75	\N	f
456881e0-a97f-410c-8a19-910a56148478	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	76	\N	f
19988e53-0493-4187-af98-7e66e8b0ccc2	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	77	\N	f
02ed9979-0fd8-4ff5-ab0a-25b43f7f48fc	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	78	\N	f
2e6c2192-e076-472b-8502-6fe24c424029	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	79	\N	f
2699ef0e-ff42-4acd-a09b-c2c5fc9b680a	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	80	\N	f
4afff015-f816-4290-91c4-1c12934acf34	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	81	\N	f
7b31acb6-6916-4ce5-b8e7-5061a9bd537d	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	82	\N	f
20915006-5aae-49f3-93df-70d2d2f7f4d7	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	83	\N	f
7c4dbe3c-1d73-45ed-bdc4-df0d4206fcf7	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	84	\N	f
db9a8ac7-f14c-4142-b81e-3bd5c95bc25d	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	85	\N	f
188e927a-f777-4e13-9609-f7d2695b5ce5	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	86	\N	f
e4a6df7f-66b6-422e-a1e9-2b3ea667ed90	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	87	\N	f
a396bea7-e0c0-43a6-ae97-e67e6b0a823c	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	88	\N	f
fcb560fe-854f-408b-9e2a-173e2f84436e	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	89	\N	f
bc13b596-4715-49b7-b8f4-ba2bda08f666	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	90	\N	f
e270b49e-044a-43b1-98ee-0079379d9a9a	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	91	\N	f
45ccabfa-8fde-4a2e-9853-eb4c55cf5c59	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	92	\N	f
dc9876bb-e41a-4a96-abd7-b536a8f1574c	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	93	\N	f
254b3d61-8ca9-4010-83b2-2abad9ac52ad	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	94	\N	f
8a3098aa-732a-42c4-a4cd-d20fc1b5864d	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	95	\N	f
198f0dce-24b0-434f-a06e-33f2e0a6077d	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	96	\N	f
677f319f-8a30-48d7-9bec-7a71a9c1e99f	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	97	\N	f
ea5cb2db-862b-4c22-adb7-2023e432412b	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	98	\N	f
5179f451-a1b5-4c2d-803f-1af59af5f0bf	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	99	\N	f
ba300b30-b5e3-45be-af96-c6e55a1064d0	1ce897cd-0bf3-4ce5-a5ee-c2fa15b271a1	100	\N	f
3b936a04-9f96-432f-b51c-c346c1156011	cce58641-86d3-42d7-8daf-40e13809844d	1	\N	f
dc4bbaa1-5440-4745-a45d-a7f7236054d1	cce58641-86d3-42d7-8daf-40e13809844d	2	\N	f
0912c443-4fd3-47e6-bd31-e109e3d805fa	cce58641-86d3-42d7-8daf-40e13809844d	3	\N	f
166759c3-8608-4c4b-b6ed-aec73611b936	cce58641-86d3-42d7-8daf-40e13809844d	4	\N	f
4c34b4e9-bf48-438e-a78b-5acc3b5c2dee	cce58641-86d3-42d7-8daf-40e13809844d	5	\N	f
a3bf81b4-1e85-438b-8efb-83ab9b4f7099	cce58641-86d3-42d7-8daf-40e13809844d	6	\N	f
47d10696-0fe6-4282-a7a3-803b90606212	cce58641-86d3-42d7-8daf-40e13809844d	7	\N	f
ba0e4a08-c588-4877-a24f-02de2dfac68d	cce58641-86d3-42d7-8daf-40e13809844d	8	\N	f
693cb821-b58b-4389-b042-4b48fd6a174f	cce58641-86d3-42d7-8daf-40e13809844d	9	\N	f
403cd956-63c7-4386-8bf2-5d4c5f281c57	cce58641-86d3-42d7-8daf-40e13809844d	10	\N	f
e484b4b3-c03a-4796-87c0-4c9876f40182	cce58641-86d3-42d7-8daf-40e13809844d	11	\N	f
9373a882-389f-4c71-b3d7-f72df898dad7	cce58641-86d3-42d7-8daf-40e13809844d	12	\N	f
02b98f17-14aa-4b82-9c95-503059f6bf6a	cce58641-86d3-42d7-8daf-40e13809844d	13	\N	f
816c6218-2189-466e-91ed-66726a8a6f68	cce58641-86d3-42d7-8daf-40e13809844d	14	\N	f
c0549c23-d37b-41aa-978b-17702c009fd0	cce58641-86d3-42d7-8daf-40e13809844d	15	\N	f
9f0d5cc7-8fba-45cd-8ded-280a05c25bf3	cce58641-86d3-42d7-8daf-40e13809844d	16	\N	f
844fd7d3-6417-4ffb-ac4c-78da956179e2	cce58641-86d3-42d7-8daf-40e13809844d	17	\N	f
17cee7f3-329f-4aaa-9a71-e3cd354671b7	cce58641-86d3-42d7-8daf-40e13809844d	18	\N	f
f6fe2242-f670-43bf-a903-7d1fafd483ed	cce58641-86d3-42d7-8daf-40e13809844d	19	\N	f
4ddd5676-02ca-4818-afee-421998c5f6f8	cce58641-86d3-42d7-8daf-40e13809844d	20	\N	f
a2f08b6a-e22c-4b3b-8f83-1d568a900843	cce58641-86d3-42d7-8daf-40e13809844d	21	\N	f
84c88635-1961-4f9d-a839-c95c57eb24f9	cce58641-86d3-42d7-8daf-40e13809844d	22	\N	f
182b8537-2dbb-4009-9021-45e1eef64ec4	cce58641-86d3-42d7-8daf-40e13809844d	23	\N	f
0302db5f-7c41-4aa9-9bc4-6e9fe2866f18	cce58641-86d3-42d7-8daf-40e13809844d	24	\N	f
b95a9e3a-881c-4c42-b3b6-df09de397656	cce58641-86d3-42d7-8daf-40e13809844d	25	\N	f
74195f5c-d08f-4437-a41d-18024e9e0ec9	cce58641-86d3-42d7-8daf-40e13809844d	26	\N	f
ceb69c12-e849-4fe0-ac1b-82f1547b16ca	cce58641-86d3-42d7-8daf-40e13809844d	27	\N	f
f69ef7e2-8782-47d7-a5d8-d588a6da925e	cce58641-86d3-42d7-8daf-40e13809844d	28	\N	f
726dd7c0-a901-4f04-a4d9-f1ecde24b481	cce58641-86d3-42d7-8daf-40e13809844d	29	\N	f
c3a68c50-f7fe-4fdf-a48f-c8cfa4d112dc	cce58641-86d3-42d7-8daf-40e13809844d	30	\N	f
7d60b44e-294d-4a60-8fc6-78f8c2c8395c	cce58641-86d3-42d7-8daf-40e13809844d	31	\N	f
b6dd833c-072e-4ae6-a70c-eb49f025d367	cce58641-86d3-42d7-8daf-40e13809844d	32	\N	f
38d74709-5ab1-4952-ad69-5fbb5c760ede	cce58641-86d3-42d7-8daf-40e13809844d	33	\N	f
3f352587-af8c-4bf7-ac13-b76d52f6b8c3	cce58641-86d3-42d7-8daf-40e13809844d	34	\N	f
a8d9d39c-a717-4fc8-be99-529fd7fd299a	cce58641-86d3-42d7-8daf-40e13809844d	35	\N	f
94bf9a9b-184f-4af0-b8c9-9c7e9374fdf7	cce58641-86d3-42d7-8daf-40e13809844d	36	\N	f
f7d318b9-6938-4d6e-be76-6a62bc79db6f	cce58641-86d3-42d7-8daf-40e13809844d	37	\N	f
cef22d24-9a98-4d71-8d39-8b6db19c27c4	cce58641-86d3-42d7-8daf-40e13809844d	38	\N	f
8f0a2c58-e4ae-4502-8e18-49de15e04b65	cce58641-86d3-42d7-8daf-40e13809844d	39	\N	f
4e913480-700a-4ba0-9ef8-b08adbab8f94	cce58641-86d3-42d7-8daf-40e13809844d	40	\N	f
4aa80438-81d7-4c21-8687-a6c0fca72b5a	cce58641-86d3-42d7-8daf-40e13809844d	41	\N	f
10d67ce1-3f87-4935-9fef-b6978fca58bf	cce58641-86d3-42d7-8daf-40e13809844d	42	\N	f
df0fa05f-22aa-4dd9-b8fc-edf7a0d6ad60	cce58641-86d3-42d7-8daf-40e13809844d	43	\N	f
74901db7-e734-4e8b-b232-887fe5500d01	cce58641-86d3-42d7-8daf-40e13809844d	44	\N	f
6ec88dee-6e61-4fed-b8c0-3771a90c7480	cce58641-86d3-42d7-8daf-40e13809844d	45	\N	f
e07f7fb5-7dab-4eb8-bfd3-a24323244395	cce58641-86d3-42d7-8daf-40e13809844d	46	\N	f
550ca635-45be-45bd-b0bc-94ce73f6cbd2	cce58641-86d3-42d7-8daf-40e13809844d	47	\N	f
04deaa12-a6a8-46b6-bf6e-b2f34949371c	cce58641-86d3-42d7-8daf-40e13809844d	48	\N	f
5d06dc9a-7ee2-4faf-8ba2-6dc7af6cbad9	cce58641-86d3-42d7-8daf-40e13809844d	49	\N	f
d460923d-b152-4eb6-b7e8-7332ba320121	cce58641-86d3-42d7-8daf-40e13809844d	50	\N	f
c618c451-dd8b-4bdc-ae2b-1a761e3453c2	2e210c58-70a5-4134-b5a8-405fb307656a	2	\N	f
da008df2-d3bc-495c-963e-cc25214412e6	2e210c58-70a5-4134-b5a8-405fb307656a	3	\N	f
cd248d94-f9b0-46b3-81b0-c7d8f2e43d05	2e210c58-70a5-4134-b5a8-405fb307656a	4	\N	f
1102f091-04f3-4f97-a865-32dbc67a3822	2e210c58-70a5-4134-b5a8-405fb307656a	5	\N	f
070430b4-f850-49d6-8982-8f757b3f6236	2e210c58-70a5-4134-b5a8-405fb307656a	6	\N	f
0db82068-e4fb-48d3-9d0e-eb6563ecb0a1	2e210c58-70a5-4134-b5a8-405fb307656a	7	\N	f
0c91ad60-d434-4901-81e8-35f1cdd3fd28	2e210c58-70a5-4134-b5a8-405fb307656a	8	\N	f
6205d9c1-d5a7-4b0f-8272-6b418da99a79	2e210c58-70a5-4134-b5a8-405fb307656a	9	\N	f
d0f8a528-0cff-4fe7-a588-68bea5c4e5a5	2e210c58-70a5-4134-b5a8-405fb307656a	10	\N	f
fa583210-6dfc-4d0b-8041-1f2fe85eef2d	2e210c58-70a5-4134-b5a8-405fb307656a	11	\N	f
7847ead8-e323-40cf-8330-df6b3385afe6	2e210c58-70a5-4134-b5a8-405fb307656a	12	\N	f
cf438fdd-8f12-4975-a51f-0dca90e23bf3	2e210c58-70a5-4134-b5a8-405fb307656a	13	\N	f
78e1865b-c338-452e-bfc9-b0db7b776e3b	2e210c58-70a5-4134-b5a8-405fb307656a	14	\N	f
853305af-f903-428a-99e8-5ff55e3f7c3e	2e210c58-70a5-4134-b5a8-405fb307656a	15	\N	f
a286fd34-4ed0-4e45-bc6d-a556fd8736d1	2e210c58-70a5-4134-b5a8-405fb307656a	16	\N	f
cfac8497-e9b1-4750-9326-b34aefacd1b1	2e210c58-70a5-4134-b5a8-405fb307656a	17	\N	f
3ca278e7-5079-4748-a72f-c8400859b5a6	2e210c58-70a5-4134-b5a8-405fb307656a	18	\N	f
14fbbebb-11a9-41fe-a426-adbe64fd2237	2e210c58-70a5-4134-b5a8-405fb307656a	19	\N	f
cf7fe65e-49a1-495e-9046-271a73a81bb8	2e210c58-70a5-4134-b5a8-405fb307656a	20	\N	f
9fecf65b-ec2a-4d4f-bc44-91b7b430fdf3	5cc241d3-1c48-4731-aa65-7d92db3e88bf	1	\N	f
7e07af13-8f54-40a8-aed9-0ba83527421f	5cc241d3-1c48-4731-aa65-7d92db3e88bf	2	\N	f
5c01afa3-a4b7-4f5b-98e6-3528360934d4	5cc241d3-1c48-4731-aa65-7d92db3e88bf	3	\N	f
243846f5-da50-413b-8de7-ee88c771f60c	5cc241d3-1c48-4731-aa65-7d92db3e88bf	4	\N	f
f0f10ba6-e26a-4e41-acc7-f71b4b45d0b9	5cc241d3-1c48-4731-aa65-7d92db3e88bf	5	\N	f
e5cd640f-96e6-49dd-8a96-8fc30e14f745	5cc241d3-1c48-4731-aa65-7d92db3e88bf	6	\N	f
504e1527-8087-4ae8-a4d8-8a0bcdb0dc68	5cc241d3-1c48-4731-aa65-7d92db3e88bf	7	\N	f
dcd01b15-3ee0-443a-8414-38fbafa255d2	5cc241d3-1c48-4731-aa65-7d92db3e88bf	8	\N	f
a2e3c78e-026c-4191-bce3-27f110471b54	5cc241d3-1c48-4731-aa65-7d92db3e88bf	9	\N	f
67de6f4f-33fc-4523-a3cd-07a4d3331000	5cc241d3-1c48-4731-aa65-7d92db3e88bf	10	\N	f
403ec3c3-92e7-4b02-bbcf-e47ac32d00cd	5cc241d3-1c48-4731-aa65-7d92db3e88bf	11	\N	f
231e034b-a54e-4a55-bdb4-204cbcb378fd	5cc241d3-1c48-4731-aa65-7d92db3e88bf	12	\N	f
c2875e63-877f-4bed-9238-37825c5ad4a5	5cc241d3-1c48-4731-aa65-7d92db3e88bf	13	\N	f
d9cc9c90-2ba0-4b8e-baf1-156e0b26fa94	5cc241d3-1c48-4731-aa65-7d92db3e88bf	14	\N	f
3848c47b-f52b-49d1-b7ba-dc7363cd30c5	5cc241d3-1c48-4731-aa65-7d92db3e88bf	15	\N	f
890ee4b0-7222-47ee-bf17-ae24c4bf391f	2e210c58-70a5-4134-b5a8-405fb307656a	1	a5436461-f586-43fe-879a-a8197a088718	t
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: rms_user
--

COPY public.users (id, role, first_name, last_name, phone_number, email, password_hash) FROM stdin;
c1e95946-137b-4b6a-a81e-c6fb5789adb0	Landlord	John	Doe	0712345678	john@doe.com	$argon2id$v=19$m=19456,t=2,p=1$CxPfQTpwsCh1i+Ri0RJxNg$X36JVIXbWwcZZ67ErAb8VIu+Jq8ID2lBQMrfhbyz6Pw
a5436461-f586-43fe-879a-a8197a088718	Tenant	Test	User	0712345678	test@user.com	$argon2id$v=19$m=19456,t=2,p=1$GH+EcS/Ni4zDPapEXlGPvg$7BCLm5f1OIryhOupEqy61C/XMSgY2aVwh9dstGewLwk
d5958480-f079-4f28-9a38-cf8983c4155d	Caretaker	Peter	Parker	0123456789	peter@parker.com	$argon2id$v=19$m=19456,t=2,p=1$p4eluqmdL/gPUEWi/vONhw$eL88sNISEdM3pWSL+09eRJBCd1f5LcaRucSsp6dpWDI
096a506b-068d-49e2-ac3e-0fc3767db658	Caretaker	Test	Taker	0123456789	test@taker.com	$argon2id$v=19$m=19456,t=2,p=1$meya2j2o8rtg3VR4yH2JLA$5MlBLA6ZBD9vRYTV/ZfaaAXnU7t6GyZI1H0w1JP9CIA
\.


--
-- Data for Name: vacation_notices; Type: TABLE DATA; Schema: public; Owner: rms_user
--

COPY public.vacation_notices (id, tenant_id, notice_date, submitted_at, status) FROM stdin;
e622217c-ba41-4192-840b-73d887d4085e	a5436461-f586-43fe-879a-a8197a088718	2026-03-10	2026-02-02 20:07:00.029604	pending
\.


--
-- Name: rent_payments_receipt_number_seq; Type: SEQUENCE SET; Schema: public; Owner: rms_user
--

SELECT pg_catalog.setval('public.rent_payments_receipt_number_seq', 1, false);


--
-- Name: buildings buildings_pkey; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.buildings
    ADD CONSTRAINT buildings_pkey PRIMARY KEY (id);


--
-- Name: caretakers caretakers_national_id_key; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.caretakers
    ADD CONSTRAINT caretakers_national_id_key UNIQUE (national_id);


--
-- Name: caretakers caretakers_pkey; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.caretakers
    ADD CONSTRAINT caretakers_pkey PRIMARY KEY (user_id);


--
-- Name: landlords landlords_pkey; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.landlords
    ADD CONSTRAINT landlords_pkey PRIMARY KEY (user_id);


--
-- Name: maintenance_requests maintenance_requests_pkey; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.maintenance_requests
    ADD CONSTRAINT maintenance_requests_pkey PRIMARY KEY (id);


--
-- Name: rent_payments rent_payments_pkey; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.rent_payments
    ADD CONSTRAINT rent_payments_pkey PRIMARY KEY (id);


--
-- Name: tenants tenants_pkey; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.tenants
    ADD CONSTRAINT tenants_pkey PRIMARY KEY (user_id);


--
-- Name: units units_pkey; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.units
    ADD CONSTRAINT units_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: vacation_notices vacation_notices_pkey; Type: CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.vacation_notices
    ADD CONSTRAINT vacation_notices_pkey PRIMARY KEY (id);


--
-- Name: buildings buildings_caretaker_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.buildings
    ADD CONSTRAINT buildings_caretaker_id_fkey FOREIGN KEY (caretaker_id) REFERENCES public.caretakers(user_id);


--
-- Name: buildings buildings_landlord_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.buildings
    ADD CONSTRAINT buildings_landlord_id_fkey FOREIGN KEY (landlord_id) REFERENCES public.users(id);


--
-- Name: caretakers caretakers_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.caretakers
    ADD CONSTRAINT caretakers_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: landlords landlords_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.landlords
    ADD CONSTRAINT landlords_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: maintenance_requests maintenance_requests_tenant_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.maintenance_requests
    ADD CONSTRAINT maintenance_requests_tenant_id_fkey FOREIGN KEY (tenant_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: maintenance_requests maintenance_requests_unit_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.maintenance_requests
    ADD CONSTRAINT maintenance_requests_unit_id_fkey FOREIGN KEY (unit_id) REFERENCES public.units(id) ON DELETE CASCADE;


--
-- Name: rent_payments rent_payments_tenant_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.rent_payments
    ADD CONSTRAINT rent_payments_tenant_id_fkey FOREIGN KEY (tenant_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: tenants tenants_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.tenants
    ADD CONSTRAINT tenants_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: units units_building_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.units
    ADD CONSTRAINT units_building_id_fkey FOREIGN KEY (building_id) REFERENCES public.buildings(id) ON DELETE CASCADE;


--
-- Name: units units_tenant_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.units
    ADD CONSTRAINT units_tenant_id_fkey FOREIGN KEY (tenant_id) REFERENCES public.users(id);


--
-- Name: vacation_notices vacation_notices_tenant_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rms_user
--

ALTER TABLE ONLY public.vacation_notices
    ADD CONSTRAINT vacation_notices_tenant_id_fkey FOREIGN KEY (tenant_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict OEgH8BqiO3MX3GGDcDefj0x4xD0sVSro4hnQZJFiN6ChZAzs9BQMXCWsPXS0rXQ

